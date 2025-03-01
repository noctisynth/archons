import {
	checkbox,
	defineCommand,
	password,
	run,
	select,
	type Context,
} from "archons";

const main = defineCommand({
	meta: {
		name: "inquire",
		styled: true,
	},
	options: {},
	callback: (ctx: Context) => {
		const color = select("What is your favorite color?", [
			"Red",
			"Green",
			"Blue",
		]);
		console.log(`Your favorite color is ${color}`);
		const foods = checkbox("What are your favorite foods?", [
			"Pizza",
			"Burger",
			"Ice Cream",
		]);
		console.log(`Your favorite foods are ${foods.join(", ")}`);
		const name = ctx.ask("What is your name?");
		console.log(`Your name is ${name}`);
		const confirm = ctx.confirm("Are you sure? [y/n]");
		console.log(`You answered ${confirm}`);
		const pwd = password("Enter your password");
		console.log(`Your password is ${pwd}`);
	},
});

run(main);
