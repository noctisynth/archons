import { type Context, defineCommand, run } from "archons";

const main = defineCommand({
	meta: {
		name: "test",
	},
	options: {
		foo: {
			type: "option",
			numArgs: "2..=3",
			required: true,
		},
	},
	callback: (_: Context) => {},
});

run(main);
