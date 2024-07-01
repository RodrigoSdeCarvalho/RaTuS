# RaTuS

RaTuS (Raft Tuple Space) is a Tuple Space implemented in Rust supported by the Raft Consensus Algorithm

![RaTuS](RaTuS/docs/ratus.webp)

## Installation (Linux or MacOS)

```bash
make install
```

## Installation (Windows)

Download [Rust Windows Installer](https://win.rustup.rs/) and run it.

## Build

```bash
make build
```

## Test

```bash
make test
```

## Troubleshooting (Linux)

In case of an error while compiling the project, install the following:

```bash
sudo apt-get update
sudo apt-get install libssl-dev
sudo apt install build-essential
```

## RaTuS test application

The test application is a simple Rust main function that creates a Raft Cluster with 3 nodes and performs some operations on the Tuple Space.

The application itself is located at `RaTuS/ratus/src/bin/test_app.rs`

In the main function of this file, there are some comments that explain the operations that are being performed and the expected results.
Also, there is a comment that explains how to run the test application as well.

Now, let's see how to run the test application:

1. Build the project:

    ```bash
    make build
    ```

2. Create 4 terminals (one for each node and one for the test application)

3. Run the first node:

    ```bash
    make start_node_1
    ```

4. Run the second node:

    ```bash
    make start_node_2
    ```

5. Run the third node:

    ```bash
    make start_node_3
    ```

6. Run the test application:

    ```bash
    make start_test_app
    ```

Now all you need to do is check the logs of the nodes and the test application to see the operations being performed and the results.
And when its time to test the fault tolerance, the test_app will warn you to stop the node 1, which is the leader.
And after that, press Enter to continue the test.
