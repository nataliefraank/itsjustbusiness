
#[derive(Resource, Entity)]
struct Task{ 
    complete: bool,
    score: i32,
}


#[derive(Entity)]
struct PlayerScore {
    taskscomplete: Vec<Tasks>,
}




impl Task {
    fn accomplish(&self){
        self.complete = true;
    }

    
}

impl Task for PlayerScore {
    fn compute_score(&self) -> i32{
        let counter = 0;
        for i in &self{
            counter += i.score;
        }

        return counter

    }
}





