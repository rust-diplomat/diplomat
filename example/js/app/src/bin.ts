import { format } from "./index.js";
import { question } from "readline-sync";

while (true) {
    const input: string = question("Enter an integer: ");
    try {
        const fmt = format(+input);
        console.log(`Formatted: ${fmt}`);
    } catch (e) {
        console.log(e.message);
    }
}
