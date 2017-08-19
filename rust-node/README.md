**Esta es la ultima parte del Taller**
**rust + node = :heart:**

En esta seccion vamos a ver como implementar node con rust, para llego en el lib.rs vamos a tener dos funciones, una que recibe parametros string y otros int.
**instalar**
para poder usar este codigo tenemos que instalar los paquetes de npm
asi que ejecutamos 

`$ npm install `

Cuando se hacen cambios en rust se debe compilar el codigo rust en la carpeta donde este el Cargo.toml

`$ cargo build`

![cargoRust](https://github.com/BerthaBrenes/TalleRust/blob/master/imgs/neonbuild.png)

Y cuando se quiera compilar todo incluyendo node

`$ neon build`

![NeonBuild](https://github.com/BerthaBrenes/TalleRust/blob/master/imgs/NeonBuildpng.png)

pero eso solo compila el codigo. Para poder usar las funciones lo hacemos desde node 

`$ node`

![node](https://github.com/BerthaBrenes/TalleRust/blob/master/imgs/node.png)

creamos una variables que nos va a ejecutar las funciones del codigo y llamamos a las funciones

**nota** debe verificar muy bien cual funcion voy a ejecutar para no llevarse sorpresas

o podemos usar por fuera de node 

`$ node -e 'require("./")'`

**nota** esto solo va a imprimir lo que hay en index.js y de acuerdo a lo que imprimi en consola no lo que la funcion vaya a retonar.
