# tree-ftp

## Installation

### Prerequisites

- Docker

### Steps

1. Clone the repository:

    ```sh
    git clone https://gitlab.univ-lille.fr/thibault.tisserand.etu/sr1-tp1-tisserand
    cd sr1-tp1-tisserand
    ```

2. Build the Docker image:

    ```sh
    docker build -t tree-ftp .
    ```

3. Start the FTP server dependency:

    ```sh
    docker run -d \
        --name ftp-server \
        -p "21:21" \
        -p 21000-21010:21000-21010 \
        -e USERS="one|1234" \
        -e ADDRESS=localhost \
        delfer/alpine-ftp-server
    ```

4. Run the `tree-ftp` command using Docker:

    ```sh
    docker run --rm --network host tree-ftp localhost -u one -p 1234
    ```
