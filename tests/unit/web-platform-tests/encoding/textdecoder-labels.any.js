// META: title=Encoding API: Encoding labels
// META: script=resources/encodings.js
// META: timeout=long

export default function ({ assert_equals, encodings_table, test }) {
	var whitespace = [" ", "\t", "\n", "\f", "\r"];
	encodings_table.forEach((section) => {
		section.encodings
			.filter((encoding) => encoding.name !== "replacement")
			.forEach((encoding) => {
				encoding.labels.forEach((label) => {
					const textDecoderName = encoding.name.toLowerCase(); // ASCII names only, so safe
					test(
						(t) => {
							assert_equals(
								new TextDecoder(label).encoding,
								textDecoderName,
								"label for encoding should match",
							);
							assert_equals(
								new TextDecoder(label.toUpperCase()).encoding,
								textDecoderName,
								"label matching should be case-insensitive",
							);
							whitespace.forEach((ws) => {
								assert_equals(
									new TextDecoder(ws + label).encoding,
									textDecoderName,
									"label for encoding with leading whitespace should match",
								);
								assert_equals(
									new TextDecoder(label + ws).encoding,
									textDecoderName,
									"label for encoding with trailing whitespace should match",
								);
								assert_equals(
									new TextDecoder(ws + label + ws).encoding,
									textDecoderName,
									"label for encoding with surrounding whitespace should match",
								);
							});
						},
						label + " => " + encoding.name,
					);
				});
			});
	});
}
