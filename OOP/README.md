 # Segunda parte del Taller

Implementación de OOP en rust como ejemplificacion de los diferentes paradigma que se pueden usar con rust

 ## Estructuras
 Son los atributos que va a contener nuestra clase
 ```bash
 struct Punto {
     x: i32,
     y: i32
 }

 struct Rectangulo {
     origen:Punto,
     ancho: i32,
     alto: i32
 }
 ```

 ## Implentaciones
 Esto se le conoce como los metodos que va a contener la clase
```bash
impl PartialEq for Rectangulo{
    fn eq(&self,other: &Rectangulo) -> bool{
        self.area() == other.area()
    }
}
impl PartialOrd for Rectangulo{
    fn partial_cmp(&self, other: &Rectangulo) -> Option<Ordering>{
        if self.area() == other.area() {
           Some(Ordering::Equal)
       }else if self.area() > other.area() {
           Some(Ordering::Greater)
       }else{
           Some(Ordering::Less)
       }
    }
}
```
## Trait (interfaces)
Los traits son más que una lista de métodos con o sin implementación que las estructuras o los tipos deben implementar para cumplir ese trait.
```bash
trait Distance {
    fn distance(&self, p: &Point) -> f32;
}

impl Distance for Point {
    fn distance(&self, p: &Point) -> f32 {
        let d1 = (self.x - p.x).pow(2);
        let d2 = (self.y - p.y).pow(2);

        f32::sqrt((d1 + d2) as f32)
    }
}
```
## Main
```bash
  let p = Punto {x: 50, y: 50};
  println!("Punto X: {}",p.x);
  let r1 = Rectangulo {origen: p,ancho: 20, alto: 20};
  println!("{}",r1);```
