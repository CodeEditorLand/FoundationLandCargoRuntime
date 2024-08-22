// META: title=Encoding API: replacement encoding
// META: script=resources/encodings.js

export default function ({ assert_throws_js, encodings_table, test }) {
	encodings_table.forEach((section) => {
		section.encodings
			.filter((encoding) => encoding.name === "replacement")
			.forEach((encoding) => {
				encoding.labels.forEach((label) => {
					test(() => {
						assert_throws_js(RangeError, () => {
							new TextDecoder(label);
						});
					}, 'Label for "replacement" should be rejected by API: ' +
						label);
				});
			});
	});
}
