// META: global=window,dedicatedworker,shadowrealm
// META: title=Encoding API: UTF-16 surrogate handling

export default function ({ assert_equals, assert_throws_js, test }) {
	var bad = [
		{
			encoding: "utf-16le",
			input: [0x00, 0xd8],
			expected: "\uFFFD",
			name: "lone surrogate lead",
		},
		{
			encoding: "utf-16le",
			input: [0x00, 0xdc],
			expected: "\uFFFD",
			name: "lone surrogate trail",
		},
		{
			encoding: "utf-16le",
			input: [0x00, 0xd8, 0x00, 0x00],
			expected: "\uFFFD\u0000",
			name: "unmatched surrogate lead",
		},
		{
			encoding: "utf-16le",
			input: [0x00, 0xdc, 0x00, 0x00],
			expected: "\uFFFD\u0000",
			name: "unmatched surrogate trail",
		},
		{
			encoding: "utf-16le",
			input: [0x00, 0xdc, 0x00, 0xd8],
			expected: "\uFFFD\uFFFD",
			name: "swapped surrogate pair",
		},
	];

	bad.forEach((t) => {
		test(
			() => {
				assert_equals(
					new TextDecoder(t.encoding).decode(new Uint8Array(t.input)),
					t.expected,
				);
			},
			t.encoding + " - " + t.name,
		);
		test(
			() => {
				assert_throws_js(TypeError, () => {
					new TextDecoder(t.encoding, { fatal: true }).decode(
						new Uint8Array(t.input),
					);
				});
			},
			t.encoding + " - " + t.name + " (fatal flag set)",
		);
	});
}
