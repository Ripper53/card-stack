pub struct Expects<T>(T);

impl<T> From<T> for Expects<(T,)> {
    fn from(value: T) -> Self {
        Expects((value,))
    }
}
impl<T> Expects<(T,)> {
    pub fn take(self) -> T {
        self.0.0
    }
}

impl<T0, T1> From<(T0, T1)> for Expects<(T0, T1)> {
    fn from(value: (T0, T1)) -> Self {
        Expects((value.0, value.1))
    }
}
impl<T0, T1> Expects<(T0, T1)> {
    pub fn take(self) -> (T0, T1) {
        (self.0.0, self.0.1)
    }
}

impl<T0, T1, T2> From<(T0, T1, T2)> for Expects<(T0, T1, T2)> {
    fn from(value: (T0, T1, T2)) -> Self {
        Expects((value.0, value.1, value.2))
    }
}
impl<T0, T1, T2> Expects<(T0, T1, T2)> {
    pub fn take(self) -> (T0, T1, T2) {
        (self.0.0, self.0.1, self.0.2)
    }
}

impl<T0, T1, T2, T3> From<(T0, T1, T2, T3)> for Expects<(T0, T1, T2, T3)> {
    fn from(value: (T0, T1, T2, T3)) -> Self {
        Expects((value.0, value.1, value.2, value.3))
    }
}
impl<T0, T1, T2, T3> Expects<(T0, T1, T2, T3)> {
    pub fn take(self) -> (T0, T1, T2, T3) {
        (self.0.0, self.0.1, self.0.2, self.0.3)
    }
}

impl<T0, T1, T2, T3, T4> From<(T0, T1, T2, T3, T4)> for Expects<(T0, T1, T2, T3, T4)> {
    fn from(value: (T0, T1, T2, T3, T4)) -> Self {
        Expects((value.0, value.1, value.2, value.3, value.4))
    }
}
impl<T0, T1, T2, T3, T4> Expects<(T0, T1, T2, T3, T4)> {
    pub fn take(self) -> (T0, T1, T2, T3, T4) {
        (self.0.0, self.0.1, self.0.2, self.0.3, self.0.4)
    }
}
