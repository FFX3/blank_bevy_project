use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(HelloPlugin)
		.run();
}


pub struct HelloPlugin;
impl Plugin for HelloPlugin{
	fn build(&self, app: &mut App){
		app
			.insert_resource(GreetTimer(Timer::from_seconds(2.0,true)))
			.add_startup_system(add_people)
			.add_system(greet_people);
	}
}

fn add_people(mut commands: Commands){
	commands.spawn().insert(Person).insert(Name("Mac".to_string()));
	commands.spawn().insert(Person).insert(Name("Joe".to_string()));
	commands.spawn().insert(Person).insert(Name("Marry".to_string()));
}

struct GreetTimer(Timer);
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>){
	if timer.0.tick(time.delta()).just_finished(){
		for name in query.iter(){
			println!("hello {}!", name.0);
		}
	}
}

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);