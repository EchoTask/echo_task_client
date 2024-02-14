# EchoTask Client

EchoTask Client is a sophisticated tool designed for users, aiming to enhance productivity and self-awareness by meticulously recording your computer activities. Through an intuitive chatbot interface, EchoTask allows you to effortlessly retrieve a summary of your daily tasks and activities, enabling better time management and productivity insights.

## Features

- **Activity Tracking**: Automatically records your activities on your macOS device, providing a detailed log of application usage.
- **Conversational Interface**: Interact with EchoTask through a user-friendly chatbot to query your activity logs.
- **Daily Summaries**: Get a concise summary of your daily activities through a simple chat command.
- **Privacy-Centric**: Designed with privacy in mind, EchoTask stores data locally on your macOS device.

## Getting Started

### Prerequisites

- macOS version 10.14 (Mojave) or later.
- Rust and Cargo installed on your machine. Visit [Rust's official site](https://www.rust-lang.org/tools/install) for installation instructions.

### Installation

1. Clone the EchoTask repository:

```bash
git clone https://github.com/EchoTask/mac-os-client.git
cd mac-os-client
```

2. Build the project using Cargo:
```bash
cargo build --release
```

3. Run EchoTask:
```bash
./target/release/echotask
```

### Usage
After launching EchoTask, it will begin recording your activity in the background. To interact with the chatbot and retrieve your activity logs or summaries, use the following commands in the chat interface:

`/summary today` - Retrieves a summary of today's activities.
`/activity [date]` - Displays detailed activity logs for a specific date (format: YYYY-MM-DD).
Replace [date] with the actual date you're interested in querying.

### Contributing
While EchoTask macOS Client is currently in a phase where active development is being undertaken solely by the project initiator, we warmly welcome discussions, feedback, and suggestions regarding the project. Feel free to open issues for discussion or suggestions on our [GitHub repository](https://github.com/EchoTask/echo_task_client). Although direct contributions to the codebase may not be integrated immediately, your insights and feedback are invaluable for shaping the future of EchoTask. We look forward to evolving the project with community input when the time is right.

### License
EchoTask macOS Client is open-source software licensed under the Apache License 2.0. For more information, please see the [LICENSE](https://github.com/EchoTask/mac-os-client?tab=Apache-2.0-1-ov-file#readme) file in the project repository.
