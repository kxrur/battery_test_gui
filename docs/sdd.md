# Software Design Document

## Overview

The Battery Qualification Bench GUI is a tool for qualifying batteries for space use. This must be used in conjunction with the actual hardware. The tool is theoretically capable of handling an infinite amount of hardware units, although only four batteries are likely to be tested at once.

## Qualification Sequence

To qualify batteries for space use they must be subjected to a charging and discharging sequence. The voltage, current, and temperature of the batteries must be logged for later analysis. The GUI is in charge of simply logging these values and conducting the following sequence:

1. Charge
2. Discharge
3. Charge
4. Discharge
5. Charge
6. Discharge

Having finished the sequence, the tool will charge the battery one last time so that it is not left discharged. The GUI is in charge of piloting the sequence and collecting data through a simple protocol described in the following section.

## Battery Cell Bench Protocol

The Battery Cell Bench Protocol is a protocol that supports talking to as many battery cell qualification benches as possible. This protocol is used to obtain data about the bench as well as command it.

### Frame Structure

All frames sent to the hardware units shall follow the following sequence of bytes:

| 0xB3 | Frame ID | Battery ID | byte 0 | ... | byte N | Checksum |
| ---- | -------- | ---------- | ------ | --- | ------ | -------- |

The first byte `0xB3` is used as a starting delimiter for the frame, the second byte `Frame ID` is a unique identification number for the frame being transmitted and the `Checksum` byte is used for frame integrity.

The frame length varies depending on the Frame ID.

### Commands

| Frame ID | Frame Description         | Length (bytes) |
| -------- | ------------------------- | -------------- |
| 0x00     | Ping                      | 4              |
| 0x01     | Assign bench ID           | 4              |
| 0x02     | Request bench data        | 16             |
| 0x04     | Set bench to standby      | 4              |
| 0x05     | Set bench to discharge    | 4              |
| 0x06     | Set bench to charge       | 4              |
| 0x07     | Announce state completion | 5              |

### Connection Sequence

Given any number of benches are allowed to connect to the GUI at one time, a connection sequence is necessary for identifying the bench itself.

Once every second, the bench will send a ping command including it's identification number. If the bench hasn't been assigned an identification number, it will instead send `0xFF`.

If the GUI receives `0xFF`, it shall generate a number between 0 and 255 and assign that number as the ID for the bench using the following command:

| 0xB3 | 0x01 | New ID | Checksum |
| ---- | ---- | ------ | -------- |

The firmware will receive the command and claim the ID. Any following Ping command received from the hardware shall be replied to with an identical frame. For example, if the hardware sends:

| 0xB3 | 0x00 | 0x23 | Checksum |
| ---- | ---- | ---- | -------- |

The GUI shall reply with:

| 0xB3 | 0x00 | 0x23 | Checksum |
| ---- | ---- | ---- | -------- |

This guarantees to the hardware that the GUI is still running. Should the GUI miss a reply by at least a second (time up to debate), the hardware will cancel its current operation and go back to a waiting state and an unknown ID.

### Data Request

The bench has three temperature sensors. One on the battery, one on the bench, and one near the electronic load. It is important to monitor these as they may get extremely hot. It is also important to monitor the battery's voltage and current.

The frame was built to send the three temperature sensor values as well as the battery voltage and current. The request is sent from the GUI with an empty payload. The payload is then replaced by the actual values and sent back by the hardware. The following is the detailed frame structure.

| 0xB3 | 0x02 | Battery ID | Battery Temp MSB | Battery Temp LSB | Bench Temp Mosfet MSB | Bench Temp Mosfet LSB | Bench Temp Resistor MSB | Bench Temp Resistor LSB | Load MSB | Load LSB | Battery Voltage MSB | Battery Voltage LSB | Bench Current MSB | Bench Current LSB | Checksum |
| ---- | ---- | ---------- | ---------------- | ---------------- | --------------------- | --------------------- | ----------------------- | ----------------------- | -------- | -------- | ------------------- | ------------------- | ----------------- | ----------------- | -------- |

The temperature values on the hardware side are multiplied by 100. Having received these values, the GUI must divide the number by 100 to obtain the proper temperature. This is called fixed-point representation.
The load values are reported in ohms.

An example with battery id: 0x23 would then be:

First the GUI requests the temperature

| 0xB3 | 0x02 | 0x23 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | 0x00 | Checksum |
| ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | -------- |

The hardware then replies

| 0xB3 | 0x02 | 0x23 | 0x07 | 0xE4 | 0x07 | 0xE4 | 0x07 | 0xE4 | 0x07 | 0xE4 | 0x00 | 0x00 | 0x00 | 0x00 | Checksum |
| ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | -------- |

The decimal value for each of these temperatures would then be `2020` which can be decoded to mean a temperature of `20.2C`. As of this time the voltage and current is still being figured out and therefore the example does not include that value.

### Selecting Battery Cell Bench State

For selecting the state of the bench, three separate commands are used. The following are the different commands.

To set the bench to standby:

| 0xB3 | 0x04 | Battery ID | Checksum |
| ---- | ---- | ---------- | -------- |

To discharge the battery:

| 0xB3 | 0x05 | Battery ID | Checksum |
| ---- | ---- | ---------- | -------- |

To charge the battery:

| 0xB3 | 0x06 | Battery ID | Checksum |
| ---- | ---- | ---------- | -------- |

The bench is able to tell when it has completed discharging or charging the battery. Upon completion, or failure in doing so, the bench will send a message to the GUI. The detailed frame structure is the following.

| 0xB3 | 0x07 | Battery ID | State and completion status | Checksum |
| ---- | ---- | ---------- | --------------------------- | -------- |

Where the `State and completion status` acts as a bit flag with the following structure:

| 0         | 1      | 2    | 3    | 4    | 5           | 6      | 7       |
| --------- | ------ | ---- | ---- | ---- | ----------- | ------ | ------- |
| Discharge | Charge | RSVD | RSVD | RSVD | In Progress | Failed | Success |

Meaning if the bench completes charging successfully for battery 0x23, it would then send the following frame:

| 0xB3 | 0x07 | 0x23 | 0x41 (0b01000001) | Checksum |
| ---- | ---- | ---- | ----------------- | -------- |

If the bench fails at discharging the battery, it would then send the following frame:

| 0xB3 | 0x07 | 0x023 | 0x82 (0b10000010) | Checksum |
| ---- | ---- | ----- | ----------------- | -------- |

## Application Architecture

Tauri is a relatively young technolgy used for creating cross-platform applications. All interactions with the operating system, shall be kept strictly to the rust section of the application. All interactions used to display values or obtain user inputs, will be kept on the frontend side.

### Backend

When talking about `Backend` we shall assume we are talking about the rust side of the application. This side is responsible for handling the communication protocol, file logging and test piloting.

#### Communication Protocol

A file `serial.rs` shall contain all helper functions and structs related to the communication protocol described previously.

As it will be seen in the following section, the battery ID assigned by the protocol is used for identifying the actual physical battery. It is important that duplicates are not assigned. To mitigate the problem, the user will be allowed to assign the ID prior to starting the test. Additionally, to obtain the latest ID used, the developer may look at the logged files.

#### File Logging

A file `file.rs` shall contain all helper functions and structs related to the logging of application errors as well as battery qualification bench results.

There are two types of files. A logging file used for logging application errors or debugging message, and a csv file containing battery data. The csv file shall be named after the battery ID that was assigned by the protocol.

#### Test Piloting

A file `pilot.rs` shall contain all helper functions and structs related to piloting the qualification sequence of the battery cells.

### Frontend

When talking about `Frotend` we shall assume we are talking about the VueJS section. This sections serves as a bridge between the user and the `Backend` such that the user may start the testing sequence, obtain battery data, and more.

To alleviate the need for styling, shadcn will be used. The only time the developer will have to create a component, is in the case of needing to combine several shadcn components.
