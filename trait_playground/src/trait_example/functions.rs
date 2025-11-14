pub trait Largest<T> {
    fn largest(&self) -> &T;
}

impl<T: PartialOrd> Largest<T> for Vec<T> {
    fn largest(&self) -> &T {
        let mut largest = &self[0];
        for item in self {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
}

impl<T: PartialOrd> Largest<T> for (T, T) {
    fn largest(&self) -> &T {
        let num1 = &self.0;
        let num2 = &self.1;
        if num1 >= num2 {
            return num1;
        } else {
            num2
        }
    }
}
