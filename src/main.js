function main() {
	console.log("-- Parsing valid JSON data --");
	loadJson(
		`{
			"name": "Fido",
			"hasSpots": false,
			"items": {
				"snacks": [
					"biscuit",
					"beef chew"
				]
			}
		}`
	);

	console.log("-- Parsing invalid JSON data --");
	loadJson(
		`
			"name": "Fido",
			"hasSpots": false,
			"items": {
				"snacks": [
					"biscuit",
					"beef chew"
				]
			}
		}`
	);
}

function loadJson(jsonString) {
	let data;
	try {
		data = JSON.parse(jsonString);
	} catch (error) {
		console.log("\tError parsing JSON: %s", error);
		data = {};
	}

	console.log("JSON data retrieved\n\t%o", data);
}

main();
