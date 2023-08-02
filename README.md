<br/>
<p align="center">
  <a href="https://github.com/vidurkhanal/accio">
    <img src="https://cdn-icons-png.flaticon.com/512/9243/9243391.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Accio</h3>

  <p align="center">
    A Fiercely Concurrent KV Database System
    <br/>
    <br/>
  </p>
</p>

![Downloads](https://img.shields.io/github/downloads/vidurkhanal/accio/total) ![Contributors](https://img.shields.io/github/contributors/vidurkhanal/accio?color=dark-green) ![Issues](https://img.shields.io/github/issues/vidurkhanal/accio) ![License](https://img.shields.io/github/license/vidurkhanal/accio) 

## About The Project

##### Accio - A High-Performance KV Database

Accio is a fast and efficient Key-Value (KV) database solution designed for high-performance data storage and retrieval. It is built using Rust programming language, leveraging innovative technologies and efficient data structures to provide developers with a reliable and scalable storage option.

##### Features

- **Lightning-Fast**: Accio is optimized for speed, ensuring minimal latency in data retrieval and storage operations, making it ideal for latency-sensitive applications.

- **Simple API**: Accio provides an intuitive and straightforward API, enabling developers to interact with the database effortlessly.

## Built With






* [Rust](<a href = "https://www.rust-lang.org/" target = "_blank"><img src="https://rustacean.net/assets/cuddlyferris.png" alt="Rust" width="200"></a>)
* [Tokio](<a href = "https://tokio.rs" target = "_blank"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/6/60/Tokio_logo.svg/1200px-Tokio_logo.svg.png" alt="Tokio" width="200"></a>)
* [Clap]( <a href = "https://docs.rs/clap/latest/clap/" target = "_blank"><img src="https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png " alt="Tokio" width="200"></a>)

## Getting Started

Here is how to start a Accio server. 

PS: By default, Accio listens on the port 55001

### Prerequisites

You need rust and its tool chain cargo to start the server.

```sh
brew install rust
```

### Installation

1. The accio-package comes with server and sample client implementation of the database using CLAP.

2. Change the directory to server

```sh
cd accio-server-rs/
```

3. Start the server to start listening on the port.

```sh
cargo run
```


## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.
* If you have suggestions for adding or removing projects, feel free to [open an issue](https://github.com/vidurkhanal/accio/issues/new) to discuss it, or directly create a pull request after you edit the *README.md* file with necessary changes.
* Please make sure you check your spelling and grammar.
* Create individual PR for each suggestion.
* Please also read through the [Code Of Conduct](https://github.com/vidurkhanal/accio/blob/main/CODE_OF_CONDUCT.md) before posting your first idea as well.

### Creating A Pull Request

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See [LICENSE](https://github.com/vidurkhanal/accio/blob/main/LICENSE.md) for more information.

## Authors

* **Vidur Khanal** - ** - [Vidur Khanal](https://www.github.com/vidurkhanal) - **

## Acknowledgements

* [ShaanCoding](https://github.com/ShaanCoding/)
* [Othneil Drew](https://github.com/othneildrew/Best-README-Template)
* [ImgShields](https://shields.io/)
