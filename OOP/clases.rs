use std::cmp::Ordering;
//consiste en ganar metodos de la estructura, de forma que derive
#[derive(Copy,Clone)] 
struct Punto 
{
    x: i32,
    y: i32
}

struct Rectangulo 
{
    origen:Punto,
    ancho: i32,
    alto: i32
}

// le implementa un metodo a la estructura rectangulo
impl Rectangulo
{
    // self hace referencia a la esctructura, y pub indica que es publico
    pub fn area(&self) -> i32
    {
        self.ancho*self.alto
    }
}
impl PartialEq for Rectangulo
{
    fn eq(&self,other: &Rectangulo) -> bool
    {
        self.area() == other.area()
    }
}
impl PartialOrd for Rectangulo
{
    fn partial_cmp(&self, other: &Rectangulo) -> Option<Ordering>
    {
        if self.area() == other.area() 
        {
           Some(Ordering::Equal)
        }
        else if self.area() > other.area() 
        {
            Some(Ordering::Greater)
        }
        else
        {
            Some(Ordering::Less)
        }
    }
}
impl std::fmt::Display for Rectangulo
{ 
    //de esta forma logro visualizar todos los datos de la funcion Rectangu;p
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f,"Origen: ({},{}) - Area: {}",self.origen.x,self.origen.y,self.area())
    }

}

fn main()
{
    let p = Punto {x: 50, y: 50};
    println!("Punto X: {}",p.x);
    let r1 = Rectangulo {origen: p,ancho: 20, alto: 20};
    println!("{}",r1);
    let r2 = Rectangulo {origen: Punto{x: 3, y: 4}, ancho: 30, alto: 30};
    println!("{}",r2);
    if r1 == r2 
    {
        println!("r2 es más grande");
    }
    else
    {
        println!("no som iguales");
    }
}
