// Module 1
pub mod audio {

    pub fn generate_sine_wave() {
        println!("generate_sine_wave");
    }

    //submodule
    pub mod filters {

        pub fn low_pass_filter() {
            println!("low_pass_filter");
        }

        pub fn high_pass_filter() {
            println!("high_pass_filter");
        }
    }

    //submodule
    pub mod effects {

        pub fn reverb() {
            println!("reverb");
        }

        pub fn delay() {
            println!("delay");
        }
    }
}

// Module 2
pub mod analysis {
    pub fn calculate_spectrum() {
        println!("calculate_spectrum");
    }

    pub fn detect_pitch() {
        println!("detect_pitch");
    }
}
