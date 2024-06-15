pub struct OnlineGradientDescent {
    learning_rate: f64,
}

impl OnlineGradientDescent {
    pub fn new(learning_rate: f64) -> Self {
        OnlineGradientDescent { learning_rate }
    }

    pub fn update(&self, weights: &mut [f64], data: &[f64], error: f64) {
        for (w, &d) in weights.iter_mut().zip(data.iter()) {
            *w -= self.learning_rate * error * d;
        }
    }
}
