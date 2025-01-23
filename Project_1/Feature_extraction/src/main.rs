use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use rand::prelude::SliceRandom;
use std::error::Error;
use linfa::traits::{Fit, PredictInplace};
use linfa::DatasetBase;
use linfa_linear::LinearRegression;
use ndarray::{array, Array1, Array2, Axis};

pub struct LinReg;

impl LinReg {
    pub fn new() -> Self {
        LinReg
    }

    /// Trains the Linear Regressor object
    pub fn train(
        &self,
        data: Array2<f64>,
        y: Array1<f64>,
    ) -> Result<linfa_linear::FittedLinearRegression<f64>, Box<dyn Error>> {
        let dataset = DatasetBase::new(data, y);
        let model = LinearRegression::default().fit(&dataset)?;
        Ok(model)
    }

    /// Return the error of the trained model
    pub fn get_fitness(
        &self,
        x: Array2<f64>,
        y: Array1<f64>,
        rng_seed: Option<u64>,
    ) -> Result<f64, Box<dyn Error>> {
        let seed = rng_seed.unwrap_or_else(|| rand::thread_rng().gen_range(0..1000));
        let mut rng = Pcg64::seed_from_u64(seed);

        let n = x.nrows();
        let test_size = (0.2 * n as f64).ceil() as usize;

        let mut indices: Vec<usize> = (0..n).collect();
        indices.shuffle(&mut rng);

        let train_indices = &indices[..n - test_size];
        let test_indices = &indices[n - test_size..];

        let x_train = x.select(Axis(0), train_indices);
        let y_train = y.select(Axis(0), train_indices);
        let x_test = x.select(Axis(0), test_indices);
        let y_test = y.select(Axis(0), test_indices);

        let model = self.train(x_train, y_train)?;

        let mut predictions = Array1::zeros(y_test.len());
        model.predict_inplace(&x_test, &mut predictions);

        let error = (predictions - y_test)
            .mapv(|val| val.powi(2))
            .mean()
            .unwrap_or(0.0)
            .sqrt();

        Ok(error)
    }

    /// Get columns of X according to bitstring
    pub fn get_columns(&self, x: Array2<f64>, bitstring: Vec<u8>) -> Array2<f64> {
        let indices: Vec<usize> = bitstring
            .iter()
            .enumerate()
            .filter_map(|(i, &val)| if val == 1 { Some(i) } else { None })
            .collect();
        x.select(Axis(1), &indices)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example data
    let data = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
    let targets = array![1.0, 2.0, 3.0];

    // Initialize LinReg
    let lin_reg = LinReg::new();

    // Train the model
    let model = lin_reg.train(data.clone(), targets.clone())?;

    // Predict
    let test_data = array![[7.0, 8.0], [9.0, 10.0]];
    let mut predictions = Array1::zeros(test_data.nrows());
    model.predict_inplace(&test_data, &mut predictions);

    println!("Predictions: {:?}", predictions);

    // Get fitness
    let fitness = lin_reg.get_fitness(data, targets, Some(42))?;
    println!("Model fitness (error): {:?}", fitness);

    Ok(())
}
