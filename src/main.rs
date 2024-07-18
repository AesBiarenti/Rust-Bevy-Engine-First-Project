use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .add_systems(Update, people_with_jobs)
        .add_systems(Update, people_ready_fire)
        .add_systems(Update,people_does_job)
        .run();
}

pub fn setup(mut commands: Commands){
    commands.spawn
    (
        (Person{name:"Kadir".to_string()},Employed{job:Job::Doctor})
    );
    commands.spawn
    (
        (Person{name:"Salih".to_string()},Employed{job:Job::Fireman})
    );
    commands.spawn
    (
        (Person{name:"Furkan".to_string()},Employed{job:Job::Officer})
    );
    commands.spawn
    (
        (Person{name:"Cengiz".to_string()},Employed{job:Job::Police})
    );
    commands.spawn
    (
        Person{name:"Kamile".to_string()}
    );
}
pub fn people_with_jobs(person_querys:Query<&Person,With<Employed>>){
    for person in person_querys.iter(){
        println!("{} Has A Job",person.name);
    }
}
pub fn print_names(person_query:Query<&Person>){
    for person in person_query.iter() {
        println!("NAME : {}", person.name)
    }
}
pub fn people_ready_fire(person_query:Query<&Person,Without<Employed>>){
    for person in person_query.iter() {
        println!("{} is ready for hire",person.name);
    }
    
}
pub fn people_does_job(person_query:Query<(&Person , &Employed)>){
    for (person,employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::Fireman => "Fireman",
            Job::Officer => "Officer",
            Job::Police => "Police",
            Job::Teacher => "Teacher",
        };
        println!("{0} is a {1}",person.name,job_name)
    }
}
#[derive(Component)]
pub struct Person{
    pub name: String,
}
#[derive(Component)]
pub struct Employed{
    pub job: Job,
}
#[derive(Component)]
pub enum Job {
    Doctor,
    Police,
    Fireman,
    Officer,
    Teacher
}