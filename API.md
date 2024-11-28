# API documentation

> [!NOTE] The long term goal for LLRT is to become
> [`Winter CG compliant`](HTTPS://github.com/wintercg/admin/blob/main/proposals.md).
> Not every API from Node.js will be supported.

## buffer

[`alloc`](HTTPS://nodejs.org/api/buffer.html#static-method-bufferallocsize-fill-encoding)

[`byteLength`](HTTPS://nodejs.org/api/buffer.html#static-method-bufferbytelengthstring-encoding)

[`concat`](HTTPS://nodejs.org/api/buffer.html#static-method-bufferconcatlist-totallength)

[`from`](HTTPS://nodejs.org/api/buffer.html#static-method-bufferfromarray)

Everything else inherited from
[`Uint8Array`](HTTPS://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array)

## child_process

> [!WARNING] > `spawn` uses native streams that is not 100% compatible with the
> Node.js Streams API.

[`spawn`](HTTPS://nodejs.org/api/child_process.html#child_processspawncommand-args-options)

## console

[`Console`](HTTPS://nodejs.org/api/console.html#class-console)

## crypto

[`createHash`](HTTPS://nodejs.org/api/crypto.html#cryptocreatehashalgorithm-options)

[`createHmac`](HTTPS://nodejs.org/api/crypto.html#cryptocreatehmacalgorithm-key-options)

[`getRandomValues`](HTTPS://nodejs.org/api/crypto.html#cryptogetrandomvaluestypedarray)

[`randomBytes`](HTTPS://nodejs.org/api/crypto.html#cryptorandombytessize-callback)

[`randomFill`](HTTPS://nodejs.org/api/crypto.html#cryptorandomfillbuffer-offset-size-callback)

[`randomFillSync`](HTTPS://nodejs.org/api/crypto.html#cryptorandomfillsyncbuffer-offset-size)

[`randomInt`](HTTPS://nodejs.org/api/crypto.html#cryptorandomintmin-max-callback)

[`randomUUID`](HTTPS://nodejs.org/api/crypto.html#cryptorandomuuidoptions)

## events

[`EventEmitter`](HTTPS://nodejs.org/api/events.html#class-eventemitter)

## fetch

Available globally

[`fetch`](HTTPS://developer.mozilla.org/en-US/docs/Web/API/fetch)

> [!IMPORTANT] There are some differences with the
> [`WHATWG standard`](HTTPS://fetch.spec.whatwg.org). Mainly browser specific
> behavior is removed:
>
> -   `keepalive` is always true
> -   `request.body` can only be `string`, `Array`, `ArrayBuffer` or
>     `Uint8Array`
> -   `response.body` returns `null`. Use `response.text()`, `response.json()`
>     etc
> -   `mode`, `credentials`, `referrerPolicy`, `priority`, `cache` is not
>     available/applicable

## file

[`file`](HTTPS://developer.mozilla.org/en-US/docs/Web/API/File)

## fs

[`accessSync`](HTTPS://nodejs.org/api/fs.html#fsaccesssyncpath-mode)

[`mkdirSync`](HTTPS://nodejs.org/api/fs.html#fsmkdirsyncpath-options)

[`mkdtempSync`](HTTPS://nodejs.org/api/fs.html#fsmkdtempsyncprefix-options)

[`readdirSync`](HTTPS://nodejs.org/api/fs.html#fsreaddirsyncpath-options)

[`readFileSync`](HTTPS://nodejs.org/api/fs.html#fsreadfilesyncpath-options)

[`rmdirSync`](HTTPS://nodejs.org/api/fs.html#fsrmdirsyncpath-options)

[`rmSync`](HTTPS://nodejs.org/api/fs.html#fsrmsyncpath-options)

[`statSync`](HTTPS://nodejs.org/api/fs.html#fsstatsyncpath-options)

[`writeFileSync`](HTTPS://nodejs.org/api/fs.html#fswritefilesyncfile-data-options)

## fs/promises

[`access`](HTTPS://nodejs.org/api/fs.html#fsstatpath-options-callback)

[`constants`](HTTPS://nodejs.org/api/fs.html#file-access-constants)

[`mkdir`](HTTPS://nodejs.org/api/fs.html#fsmkdirpath-options-callback)

[`mkdtemp`](HTTPS://nodejs.org/api/fs.html#fsmkdtempprefix-options-callback)

[`readdir`](HTTPS://nodejs.org/api/fs.html#fspromisesreaddirpath-options)

[`readFile`](HTTPS://nodejs.org/api/fs.html#filehandlereadfileoptions)

[`rm`](HTTPS://nodejs.org/api/fs.html#fsrmpath-options-callback)

[`rmdir`](HTTPS://nodejs.org/api/fs.html#fsrmdirpath-options-callback)

[`stat`](HTTPS://nodejs.org/api/fs.html#fsstatpath-options-callback)

[`writeFile`](HTTPS://nodejs.org/api/fs.html#fspromiseswritefilefile-data-options)

## module

[`createRequire`](HTTPS://nodejs.org/api/module.html#modulecreaterequirefilename)

> [!NOTE] > `require` is available from esm modules natively. This function is
> just for compatibility

## net

> [!WARNING] These APIs uses native streams that is not 100% compatible with the
> Node.js Streams API. Server APIs like `createSever` provides limited
> functionality useful for testing purposes. Serverless applications typically
> don't expose servers. Some server options are not supported: `highWaterMark`,
> `pauseOnConnect`, `keepAlive`, `noDelay`, `keepAliveInitialDelay`

[`connect`](HTTPS://nodejs.org/api/net.html#netconnect)

[`createConnection`](HTTPS://nodejs.org/api/net.html#netcreateconnection)

[`createServer`](HTTPS://nodejs.org/api/net.html#netcreateserveroptions-connectionlistener)

## os

[`platform`](HTTPS://nodejs.org/api/os.html#osplatform)

[`release`](HTTPS://nodejs.org/api/os.html#osrelease)

[`tmpdir`](HTTPS://nodejs.org/api/os.html#osplatform)

[`type`](HTTPS://nodejs.org/api/os.html#ostype)

## path

[`basename`](HTTPS://nodejs.org/api/path.html#pathbasenamepath-suffix)

[`delimiter`](HTTPS://nodejs.org/api/path.html#pathdelimiter)

[`dirname`](HTTPS://nodejs.org/api/path.html#pathdirnamepath)

[`extname`](HTTPS://nodejs.org/api/path.html#pathextnamepath)

[`format`](HTTPS://nodejs.org/api/path.html#pathformatpathobject)

[`isAbsolute`](HTTPS://nodejs.org/api/path.html#pathisabsolutepath)

[`join`](HTTPS://nodejs.org/api/path.html#pathjoinpaths)

[`normalize`](HTTPS://nodejs.org/api/path.html#pathnormalizepath)

[`parse`](HTTPS://nodejs.org/api/path.html#pathparsepath)

[`resolve`](HTTPS://nodejs.org/api/path.html#pathresolvepaths)

## timers

_Also available globally_

[`clearImmediate`](HTTPS://nodejs.org/api/timers.html#clearimmediateimmediate)

[`clearInterval`](HTTPS://nodejs.org/api/timers.html#clearintervaltimeout)

[`clearTimeout`](HTTPS://nodejs.org/api/timers.html#cleartimeouttimeout)

[`setImmediate`](HTTPS://nodejs.org/api/timers.html#setimmediatecallback-args)

[`setInterval`](HTTPS://nodejs.org/api/timers.html#setintervalcallback-delay-args)

[`setTimeout`](HTTPS://nodejs.org/api/timers.html#settimeoutcallback-delay-args)

## url

```typescript
export class URL {
  constructor(input: string, base?: string | URL);

  hash: string;
  host: string;
  hostname: string;
  href: string;
  origin: string;
  password: string;
  pathname: string;
  port: string;
  protocol: string;
  search: string;
  searchParams: URLSearchParams;
  username: string;

  parse(input: string, base?: string): URL | null;
  canParse(input: string, base?: string): boolean;
  toJSON(): string;
  toString(): string;
}
```

```typescript
// Additional utilities in the URL module
export function domainToASCII(domain: string): string;

export function domainToUnicode(domain: string): string;

export function fileURLToPath(url: string | URL): string;

export function pathToFileURL(path: string): URL;

export function format(url: string | URL, options?: { fragment?: boolean, unicode?: boolean, auth?: boolean
}): string;

export function urlToHttpOptions(url: URL): {
  protocol?: string;
  hostname?: string;
  port?: string;
  path?: string;
  ...
};
```

## URLSearchParams

```typescript
export class URLSearchParams {
  constructor(
    init?: string | string[``][] | Record<string, string> | URLSearchParams
  );

  // properties
  size: number;

  // Methods
  append(name: string, value: string): void;
  delete(name: string): void;
  get(name: string): string | null;
  getAll(name: string): string[];
  has(name: string): boolean;
  set(name: string, value: string): void;
  sort(): void;

  [`Symbol.iterator`](): IterableIterator<[string, string]>;
  entries(): IterableIterator<[string, string]>;
  forEach(): IterableIterator<[string, string]>;
  keys(): IterableIterator<string>;
  values(): IterableIterator<string>;

  toString(): string;
}
```

## util

> [!IMPORTANT] Supported encodings: hex, base64, utf-8, utf-16le, windows-1252
> and their aliases.

[`TextDecoder`](HTTPS://nodejs.org/api/util.html#class-utiltextdecoder)

[`TextEncoder`](HTTPS://nodejs.org/api/util.html#class-utiltextdecoder)

## zlib

[`deflate`](HTTPS://nodejs.org/api/zlib.html#zlibdeflatebuffer-options-callback)

[`deflateSync`](HTTPS://nodejs.org/api/zlib.html#zlibdeflatesyncbuffer-options)

[`deflateRaw`](HTTPS://nodejs.org/api/zlib.html#zlibdeflaterawbuffer-options-callback)

[`deflateRawSync`](HTTPS://nodejs.org/api/zlib.html#zlibdeflaterawsyncbuffer-options)

[`gzip`](HTTPS://nodejs.org/api/zlib.html#zlibgzipbuffer-options-callback)

[`gzipSync`](HTTPS://nodejs.org/api/zlib.html#zlibgzipsyncbuffer-options)

[`inflate`](HTTPS://nodejs.org/api/zlib.html#zlibinflatebuffer-options-callback)

[`inflateSync`](HTTPS://nodejs.org/api/zlib.html#zlibinflatesyncbuffer-options)

[`inflateRaw`](HTTPS://nodejs.org/api/zlib.html#zlibinflaterawbuffer-options-callback)

[`inflateRawSync`](HTTPS://nodejs.org/api/zlib.html#zlibinflaterawsyncbuffer-options)

[`gunzip`](HTTPS://nodejs.org/api/zlib.html#zlibgunzipbuffer-options-callback)

[`gunzipSync`](HTTPS://nodejs.org/api/zlib.html#zlibgunzipsyncbuffer-options)

[`brotliCompress`](HTTPS://nodejs.org/api/zlib.html#zlibbrotlicompressbuffer-options-callback)

[`brotliCompressSync`](HTTPS://nodejs.org/api/zlib.html#zlibbrotlicompresssyncbuffer-options)

[`brotliDecompress`](HTTPS://nodejs.org/api/zlib.html#zlibbrotlidecompressbuffer-options-callback)

[`brotliDecompressSync`](HTTPS://nodejs.org/api/zlib.html#zlibbrotlidecompresssyncbuffer-options)

## llrt:hex

```typescript
export function encode(
  value: string | Array | ArrayBuffer | Uint8Array
): string;
export function decode(value: string): Uint8Array;
```

## llrt:uuid

```typescript
export const NIL: string;

export function v1(): string;

export function v3(
  name: string,
  namespace: Array | Uint8Array | String
): string;

export function v4(): string;

export function v5(
  name: string,
  namespace: Array | Uint8Array | String
): string;

export function parse(value: string): Uint8Array;

export function stringify(arr: Array | Uint8Array): string;

export function validate(arr: string): boolean;

export function version(arr: Array | Uint8Array): number;
```

## llrt:xml

A lightweight and fast XML parser

```typescript
type XmlParserOptions = {
    ignoreAttributes?: boolean;
    attributeNamePrefix?: string;
    textNodeName?: string;
    attributeValueProcessor?: (attrName: string, attrValue: string, jpath: string) => unknown;
    tagValueProcessor?: (attrName: string, attrValue: string, jpath: string, hasAttributes: boolean) => unknown;
}
export class XMLParser(options?: XmlParserOptions){
    parse(xml:string):object
}
```

## Misc Global objects

[`AbortController`](HTTPS://developer.mozilla.org/en-US/docs/Web/API/AbortController)

[`AbortSignal`](HTTPS://developer.mozilla.org/en-US/docs/Web/API/AbortSignal)

[`atob`](HTTPS://developer.mozilla.org/en-US/docs/Web/API/atob)

[`btoa`](HTTPS://developer.mozilla.org/en-US/docs/Web/API/btoa)

[`DOMException`](HTTPS://developer.mozilla.org/en-US/docs/Web/API/DOMException)

[`navigator.userAgent`](HTTPS://nodejs.org/api/globals.html#navigatoruseragent)

[`performance.now`](HTTPS://nodejs.org/api/perf_hooks.html#performancenow)

[`performance.timeOrigin`](HTTPS://nodejs.org/api/perf_hooks.html#performancetimeorigin)

[`structuredClone`](HTTPS://nodejs.org/api/globals.html#structuredclonevalue-options)
