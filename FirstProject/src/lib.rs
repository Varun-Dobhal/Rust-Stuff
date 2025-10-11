mod diet{
    const NUTRITIONST : &str="Norah Nutrition";
    
    pub fn ask_about_program(){
        println!("The Nutritionist is {}",NUTRITIONST);
    }
}

mod weightlifting;
mod cardio;

use cardio::{CardioTool,Exercise as CardioExercise };
use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout{
    cardio:CardioExercise,
    weightlifting:WeightliftingExercise,
}

impl GymWorkout{
    pub fn new()->Self{
        diet::ask_about_program();
        weightlifting::ask_about_program();
        cardio::ask_about_program();

        Self { 
            cardio: CardioExercise { day:String::from("Saturday"), tool:CardioTool::Treadmill, minutes:30 },
            weightlifting: WeightliftingExercise{name:String::from("Varun"), reps:10},
        }
    }
}