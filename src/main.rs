use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::nalgebra::Point2 as P2;

struct MainState {
	pos_x: f32,
	len: f32,
	theta: f32,
}

impl MainState {
	fn new() -> ggez::GameResult<MainState> {
		let s = MainState { 
			pos_x: 0.0,
			len: 360.0,
			theta: 0.0,
		};
		Ok(s)
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
		self.pos_x = self.pos_x % 800.0 + 1.0;
		self.theta = self.theta % 360.0 +0.5;
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

		let point_bob = P2::new(
			self.theta.to_radians().sin() * self.len + self.len,
			self.theta.to_radians().cos() * self.len
		);

		let bob = graphics::Mesh::new_circle(
		    ctx,
		    graphics::DrawMode::fill(),
		    point_bob,
		    100.0,
		    2.0,
		    graphics::WHITE,
		)?;
		let mut points = [
			//P2::new(0.0,0.0),
			na::Point2::new(360.0,0.0),
			//na::Point2::new(0.0,30.0),
			point_bob
		];
		
		let string = graphics::Mesh::new_line(
			ctx,
			&mut points,
			15.0,
			graphics::WHITE,
		)?;
		graphics::draw(ctx, &string, graphics::DrawParam::new())?;
		graphics::draw(ctx, &bob, graphics::DrawParam::new())?;
		graphics::present(ctx)?;
		Ok(())
	}
}

pub fn main() -> ggez::GameResult { 
	let cb = ggez::ContextBuilder::new("super_simple", "ggez");
	let (ctx, event_loop) = &mut cb.build()?;
	let state = &mut MainState::new()?;
	event::run(ctx, event_loop, state)
}