// Wokwi Custom Chip - For docs and examples see:
// https://docs.wokwi.com/chips-api/getting-started
//
// SPDX-License-Identifier: MIT
// Copyright 2025 The Embedded Rustacean

#include "wokwi-api.h"
#include <stdio.h>
#include <stdlib.h>

#define I2C_BASE_ADDRESS 0x22
#define NUM_GPIO 24
#define NUM_ADDR_BITS 1

#define CHIPSTATE_FROM(usr_dat) chip_state_t * chip = (chip_state_t*)usr_dat
#define PRINTF_INPUTVALUE(msg, val)   printf("%s 0x%x (p0 0x%x, p1 0x%x, p2 0x%x)", msg, val, (val & 0xff), ((val & 0xff00) >> 8), ((val & 0xff0000) >> 16))

/* Chip state structure */
typedef struct {
  // address bit pin and device address
  uint8_t address;
  pin_t addressBit;

  // interrupt, reset, and I/O pins
  pin_t nINT;
  pin_t reset;
  pin_t io[NUM_GPIO];

  // register banks
  uint32_t inputMask;          // Configuration: 1 = input, 0 = output
  uint32_t inputValue;         // Input register
  uint32_t lastReadValue;      // Last read input value
  uint32_t outputValue;        // Output register
  uint32_t polarityInversion;  // Polarity inversion: 1 = inverted, 0 = normal

  // i2c configuration, device, and tracking
  i2c_dev_t i2c_dev;
  i2c_config_t i2c_config;
  uint8_t i2c_portcount;
  uint8_t current_register;     // Current register for reads/writes
  bool auto_increment;         // Auto-increment flag

  // pin watch config for I/O
  pin_watch_config_t io_watch_config;
} chip_state_t;

/* Interrupt flag control 
  Note: inverted logic, i.e. when interrupt is asserted
  the open-drain output is a "switch" tied to ground.
  Otherwise, it is floating.
*/
void interruptFlagOff(chip_state_t* chip) {
  pin_mode(chip->nINT, INPUT);
}

void interruptFlagOn(chip_state_t* chip) {
  pin_mode(chip->nINT, OUTPUT_LOW);
  printf("INT flag SET\n");
}

void chip_reset(void *user_data, pin_t pin, uint32_t value) {
  CHIPSTATE_FROM(user_data);
  if (!value) { // Active low
    chip->inputMask = 0xffffff; // All pins input
    chip->outputValue = 0;
    chip->polarityInversion = 0;
    chip->inputValue = 0;
    chip->lastReadValue = 0;
    chip->current_register = 0;
    chip->i2c_portcount = 0;
    chip->auto_increment = true;

    for (uint8_t i = 0; i < NUM_GPIO; i++) {
      pin_watch_stop(chip->io[i]);
      pin_mode(chip->io[i], INPUT_PULLUP);
      pin_watch(chip->io[i], &(chip->io_watch_config));
    }

    interruptFlagOff(chip);
    printf("RESET triggered\n");
  }
}


/*
  All transactions are to be three-bytes long.
  Anything beyond this, in multiples of three, 
  simply restarts/overwrites, like a circular 
  buffer.
*/
void incrementPortCount(chip_state_t* chip) {
  if (chip->auto_increment) {
    uint8_t current_port = (chip->current_register & 0x03);
    uint8_t next_port = (current_port + 1) % 3; // Cycle 0 -> 1 -> 2 -> 0
    chip->current_register = (chip->current_register & 0xFC) | next_port; // Update port bits
  }
}

/*
  I2C connection callback.
  Will tell up the device address and whether this is a read or write 
  operation
*/
// bool on_i2c_connect(void *user_data, uint32_t address, bool read) {
//   CHIPSTATE_FROM(user_data);

//   if (address != chip->address) {
//     printf("Getting connects for address 0x%x but am at 0x%x\n", address, chip->address);
//     return false; // NACK if address doesn't match
//   }

//   chip->i2c_portcount = 0;
//   chip->current_register = 0; // Default to Input Port 0
//   chip->auto_increment = true; // Default AI bit to 1

//   if (read) {
//     printf("Read: reset INT flag\n");
//     chip->lastReadValue = chip->inputValue;
//     interruptFlagOff(chip);
//   }
//   return true; // ACK
// }
bool on_i2c_connect(void *user_data, uint32_t address, bool read) {
  CHIPSTATE_FROM(user_data);

  if (address != chip->address) {
    printf("Getting connects for address 0x%x but am at 0x%x\n", address, chip->address);
    return false; // NACK if address doesn't match
  }

  if (read) {
    // This is a READ operation.
    // DO NOT reset the current_register. It was set by the previous write.
    printf("Read: reset INT flag\n");
    chip->lastReadValue = chip->inputValue;
    interruptFlagOff(chip);
  } else {
    // This is a WRITE operation. Reset pointers.
    chip->i2c_portcount = 0;
    chip->current_register = 0; // Default to Input Port 0
    chip->auto_increment = true; // Default AI bit to 1
  }
  return true; // ACK
}

/*
  Reading a port as a single byte.
*/
uint8_t on_i2c_read(void *user_data) {
  CHIPSTATE_FROM(user_data);
  uint8_t retVal = 0;

  uint8_t reg_bank = (chip->current_register & 0x0C) >> 2; // Bits B3:B2
  uint8_t port = (chip->current_register & 0x03); // Use B1:B0 from stored command

  if (reg_bank == 0x00) { // Input Port
    retVal = ((chip->inputValue >> (port * 8)) & 0xff);
    if (chip->polarityInversion & (1 << (port * 8))) {
      retVal ^= 0xff; // Apply polarity inversion
    }
  } else if (reg_bank == 0x01) { // Output Port
    retVal = (chip->outputValue >> (port * 8)) & 0xff;
  } else if (reg_bank == 0x02) { // Polarity Inversion
    retVal = (chip->polarityInversion >> (port * 8)) & 0xff;
  } else if (reg_bank == 0x03) { // Configuration
    retVal = (chip->inputMask >> (port * 8)) & 0xff;
  }

  incrementPortCount(chip);
  return retVal;
}

/*
  Writing to a port configures the pins as either
   * output LOW
   * output HIGH / Input (with pull-up)
*/
bool on_i2c_write(void *user_data, uint8_t data) {
  CHIPSTATE_FROM(user_data);

  if (chip->i2c_portcount == 0 && chip->current_register == 0) {
    // Command byte
    chip->auto_increment = (data & 0x80) != 0; // AI bit (B7)
    chip->current_register = (data & 0x0F); // B3:B0 select register
    return true; // ACK command byte
  }

  uint8_t reg_bank = (chip->current_register & 0x0C) >> 2; // Bits B3:B2
  // uint8_t port = chip->i2c_portcount;
  uint8_t port = (chip->current_register & 0x03); // Use B1:B0 from stored command
  uint32_t shift = port * 8;

  if (reg_bank == 0x00) { // Input Port (read-only, ignore writes)
    // No action
  } else if (reg_bank == 0x01) { // Output Port
    chip->outputValue = (chip->outputValue & ~(0xff << shift)) | ((uint32_t)data << shift);
    for (uint8_t i = 0; i < 8; i++) {
      uint8_t bitIdx = i + shift;
      if (!(chip->inputMask & (1 << bitIdx))) { // Only update outputs
        pin_t targetPin = chip->io[bitIdx];
        pin_mode(targetPin, (data & (1 << i)) ? OUTPUT_HIGH : OUTPUT_LOW);
      }
    }
  } else if (reg_bank == 0x02) { // Polarity Inversion
    chip->polarityInversion = (chip->polarityInversion & ~(0xff << shift)) | ((uint32_t)data << shift);
  } else if (reg_bank == 0x03) { // Configuration
    uint32_t new_input_mask = (chip->inputMask & ~(0xff << shift)) | ((uint32_t)data << shift);
    for (uint8_t i = 0; i < 8; i++) {
      uint8_t bitIdx = i + shift;
      pin_t targetPin = chip->io[bitIdx];
      pin_watch_stop(targetPin);
      if (data & (1 << i)) {
        chip->inputMask |= (1 << bitIdx);
        pin_mode(targetPin, INPUT_PULLUP);
        pin_watch(targetPin, &(chip->io_watch_config));
      } else {
        chip->inputMask &= ~(1 << bitIdx);
        pin_mode(targetPin, (chip->outputValue & (1 << bitIdx)) ? OUTPUT_HIGH : OUTPUT_LOW);
      }
    }
    chip->inputMask = new_input_mask;
  }

  printf("Write to reg 0x%x, port %d: 0x%x\n", chip->current_register, port, data);
  incrementPortCount(chip);
  return true;
}


void on_i2c_disconnect(void *user_data) {
  CHIPSTATE_FROM(user_data);
  chip->i2c_portcount = 0;
  chip->current_register = 0;
}


/*  I2C address management
  This is only available on start-up/init, so could in theory 
  simply be handled there.
*/
uint8_t read_address(chip_state_t* chip) {
  uint8_t configuredAddress = I2C_BASE_ADDRESS;
  if (pin_read(chip->addressBit)) {
    configuredAddress |= 0x01; // ADDR high sets address to 0x23
  }

  printf("Chip ADDR pin %s. Address is 0x%02x\n", pin_read(chip->addressBit) ? "HIGH" : "LOW", configuredAddress);

  return configuredAddress;
}


void chip_addr_change(void *user_data, pin_t pin, uint32_t value) {
  CHIPSTATE_FROM(user_data);
  chip->address = read_address(chip);
  chip->i2c_config.address = chip->address;
  i2c_init(&(chip->i2c_config)); // Reinitialize I2C with new address
}



/* 
  Read the value of the combined input pins, skipping over 
  any that have been configured as output (LOW)
*/
uint32_t readInputsValue(chip_state_t * chip) {
  uint32_t inputsValue = 0;
  for (uint8_t i = 0; i < NUM_GPIO; i++) {
    if (chip->inputMask & (1 << i)) {
      bool pinValue = pin_read(chip->io[i]);
      if (chip->polarityInversion & (1 << i)) {
        pinValue = !pinValue; // Apply polarity inversion
      }
      if (pinValue) {
        inputsValue |= (1 << i);
      }
    }
  }
  return inputsValue;
}


/*
  Anything that _may_ be treated as an output is watched and,
  on changes, this callback is triggered.
  If the value read in is different than that provided to user
  on last read, we will set the interrupt flag.
  If it is the same, we _clear_ the interrupt flag--this means
  that some changes may be missed by user... yap, but that's 
  how the chip works.
*/
void chip_input_io_change(void *user_data, pin_t pin, uint32_t value) {
  CHIPSTATE_FROM(user_data);
  
  chip->inputValue = readInputsValue(chip);

  if (chip->inputValue != chip->lastReadValue) {
    interruptFlagOn(chip);
  } else {
    interruptFlagOff(chip);
  }

  printf("I/O input changed:");
  PRINTF_INPUTVALUE("from", chip->lastReadValue);
  PRINTF_INPUTVALUE("to", chip->inputValue);
  printf("\n");
}

/*
  The chip_state_t structure is initialized here.
  Could do this in chip_init, but there are lots of values
  to set so this is separated out just for clarity.
*/
void initialize_state(chip_state_t * chip) {
  chip->address = I2C_BASE_ADDRESS;
  chip->inputMask = 0xffffff; // All pins input by default
  chip->inputValue = 0;
  chip->lastReadValue = 0;
  chip->outputValue = 0;
  chip->polarityInversion = 0;
  chip->i2c_portcount = 0;
  chip->current_register = 0;
  chip->auto_increment = true; // Default AI=1

  chip->io_watch_config.edge = BOTH;
  chip->io_watch_config.pin_change = chip_input_io_change;
  chip->io_watch_config.user_data = chip;

  i2c_config_t * i2c = &(chip->i2c_config);
  i2c->scl = pin_init("SCL", INPUT_PULLUP);
  i2c->sda = pin_init("SDA", INPUT_PULLUP);
  
  i2c->connect = on_i2c_connect;
  i2c->read = on_i2c_read;
  i2c->write = on_i2c_write;
  i2c->disconnect = on_i2c_disconnect;

  i2c->user_data = chip;
}

/*
  Chip initialization, called on startup.
*/
void chip_init() {
  chip_state_t *chip = malloc(sizeof(chip_state_t));
  const char * ioPinNames[] = {
    "P00", "P01", "P02", "P03", "P04", "P05", "P06", "P07",
    "P10", "P11", "P12", "P13", "P14", "P15", "P16", "P17",
    "P20", "P21", "P22", "P23", "P24", "P25", "P26", "P27"};

  initialize_state(chip);

  chip->nINT = pin_init("INT", INPUT);
  chip->reset = pin_init("RESET", INPUT);
  chip->addressBit = pin_init("ADDR", INPUT);

  const pin_watch_config_t watch_addr_config = {
    .edge = BOTH,
    .pin_change = chip_addr_change,
    .user_data = chip,
  };
  pin_watch(chip->addressBit, &watch_addr_config);

  const pin_watch_config_t watch_reset_config = {
    .edge = FALLING,
    .pin_change = chip_reset,
    .user_data = chip,
  };
  pin_watch(chip->reset, &watch_reset_config);

  for (uint8_t i = 0; i < NUM_GPIO; i++) {
    chip->io[i] = pin_init(ioPinNames[i], INPUT_PULLUP);
    pin_watch(chip->io[i], &(chip->io_watch_config));
  }

  chip->address = read_address(chip);
  chip->i2c_config.address = chip->address;
  chip->i2c_dev = i2c_init(&(chip->i2c_config));

  // Read initial pin states
  chip->inputValue = readInputsValue(chip);

  printf("I2C initialized @ address 0x%x\n", chip->address);

  interruptFlagOff(chip);
}





