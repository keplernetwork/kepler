# Kepler - Compilación, configuración y ejecución

## Plataformas soportadas

En un largo plazo, es probable que la mayoría de las plataformas sean compatibles en cierta medida.
El lenguaje de programación de Kepler `rust` ha compilado metas para la mayoría de las plataformas.

¿Qué funciona hasta ahora?

* Linux x86\_64 y MacOS [kepler + mining + development]
* Todavía no funciona con windows 10 [kepler kind-of builds. No mining yet. Help wanted!]

## Requisitos

* rust 1.34+ (usa [rustup]((https://www.rustup.rs/))- por ejemplo, `curl https://sh.rustup.rs -sSf | sh; source $HOME/.cargo/env`)
  * Si rust está instalado, puede simplemente actualizar la versión con  `rustup update`
* clang
* ncurses y libs (ncurses, ncursesw5)
* zlib libs (zlib1g-dev or zlib-devel)
* pkg-config
* libssl-dev
* linux-headers (reportado como necesario en Alpine linux)
* llvm

Para las distribuciones basadas en Debian (Debian, Ubuntu, Mint, etc), todo en un comando (exceptuando Rust):

```sh
apt install build-essential cmake git libgit2-dev clang libncurses5-dev libncursesw5-dev zlib1g-dev pkg-config libssl-dev llvm
```

Para las Mac:

```sh
xcode-select --install
brew install --with-toolchain llvm
brew install pkg-config
brew install openssl
```

## Pasos para la compilación

```sh
git clone https://github.com/keplernetwork/kepler.git
cd kepler
cargo build --release
```

Kepler también puede compilarse en modo debug (sin la etiqueta `--release`, pero usando la etiqueta `--debug` o `--verbose`) esto hará que la sincronización rápida sea excesivamente lenta debido a la gran sobrecarga de las operaciones criptográficas.

## Errores de compilación

Vea [Solución de problemas](https://github.com/keplernetwork/docs/wiki/Troubleshooting)

## ¿Qué se ha compilado?

Con una compilación finalizada se obtiene:

* `target/release/kepler` - los binarios principales de kepler

Todos los datos, configuración y archivos de registro creados y utilizados por Kepler se encuentran en el directorio oculto `~/.kepler` (bajo el directorio home del usuario) por defecto. Puede modificar toda la configuración editando el archivo `~/.kepler/main/kepler-server.toml`.

También es posible hacer que Kepler cree sus propios archivos de datos en el directorio actual. Para ello ejecute:

```sh
kepler server config
```

Lo que generará un archivo `kepler-server.toml` en el directorio actual, preconfigurado para usar el directorio actual para todos sus datos. Ejecutando Kepler desde un directorio que contiene el archivo `kepler-server.toml` usará los valores de ese archivo en lugar de los valores por defecto de `~/.kepler/main/kepler-server.toml`.

Durante las pruebas, ponga el binario de Kepler en su ruta de esta manera:

```sh
export PATH=/path/to/kepler/dir/target/release:$PATH
```

Donde `path/to/kepler/dir` es su ruta absoluta al directorio raíz de la instalación de Kepler.

Puede ejecutar `kepler` directamente (pruebe `kepler help` para más opciones).

## Configuración

Kepler se ejecuta con valores predeterminados, y puede configurarse aún más a través del archivo `kepler-server.toml`. Este fichero es generado por kepler en su primera ejecución, y contiene documentación sobre cada opción disponible.

Aunque se recomienda que realice toda la configuración de kepler server a través de `kepler-server.toml`, también es posible suministrar cambios de comandos para kepler que anulan cualquier configuración en el archivo.

Para obtener ayuda sobre los comandos de kepler y sus cambios intente:

```sh
kepler help
kepler wallet help
kepler client help
```

## Docker

```sh
docker build -t kepler -f etc/Dockerfile .
```

Puede ubicar la caché de Kepler para que se ejecute dentro del contenedor

```sh
docker run -it -d -v $HOME/.kepler:/root/.kepler kepler
```
## Compilación multiplataforma

Rust (cargo) puede compilar Kepler para muchas plataformas, así que en teoría ejecutar `kepler` como un nodo de validación en un dispositivo de baja potencia podría ser posible. Para hacer una compilación cruzada `kepler` en una plataforma x86 Linux y generar binarios de ARM, por ejemplo para Raspberry-pi.

## Usando Kepler

La página de la wiki [Cómo usar kepler](https://github.com/keplernetwork/docs/wiki/How-to-use-kepler) y las páginas de enlaces tienen más información sobre las características que disponemos, resolución de problemas, etc.

## Minando en Kepler

Tenga en cuenta que todas las funciones de minería de Kepler se han trasladado a un paquete independiente llamado [kepler_minner](https://github.com/keplernetwork/kepler-miner). Una vez que el nodo de kepler esté listo y funcionando, puede empezar a minar compilando y ejecutando kepler-miner con su nodo Kepler en funcionamiento.
