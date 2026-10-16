<?php

declare(strict_types=1);

/*
 * Exports nette/neon's own test suite (.oracle/neon/tests/Neon/*.phpt) as fixtures for crates/pun-phpstan:
 * every Neon::decode()/decodeFile()/encode() call the tests make is recorded with nette/neon's real result.
 *
 *   php scripts/export-neon-fixtures.php .oracle/neon > crates/pun-phpstan/tests/fixtures/neon.json
 *   php scripts/export-neon-fixtures.php .oracle/neon extra-inputs.json   # decode a JSON list of strings instead
 *
 * Values are tagged JSON (see crates/pun-phpstan/tests/common/mod.rs): null/bool/int/string as JSON,
 * {"float": var_export}, {"array": [[key, value], ...]}, {"entity": v, "attributes": a}, {"datetime": "Y-m-d H:i:s.u P"},
 * {"object": class, "props": a}. Timezone is UTC (php.ini default; PHPStan uses whatever php.ini says).
 */

namespace {
    $root = realpath($argv[1]) ?: exit("usage: export-neon-fixtures.php <nette/neon checkout>\n");
    date_default_timezone_set('UTC');
    spl_autoload_register(static function (string $class) use ($root): void {
        if ($class === 'Nette\Neon\Neon') {
            return; // the recording wrapper below
        }
        if (str_starts_with($class, 'Nette\Neon\\')) {
            $f = $root.'/src/Neon/'.str_replace('\\', '/', substr($class, 11)).'.php';
            if (is_file($f)) {
                require $f;
            }
        }
    });

    function tagged(mixed $v): mixed
    {
        if (is_float($v)) {
            return ['float' => var_export($v, true)];
        }
        if (is_array($v)) {
            $items = [];
            foreach ($v as $k => $x) {
                $items[] = [$k, tagged($x)];
            }

            return ['array' => $items];
        }
        if ($v instanceof Nette\Neon\Entity) {
            return ['entity' => tagged($v->value), 'attributes' => tagged($v->attributes)];
        }
        if ($v instanceof DateTimeInterface) {
            return ['datetime' => $v->format('Y-m-d H:i:s.u P')];
        }
        if (is_object($v)) {
            return ['object' => get_class($v), 'props' => tagged((array) $v)];
        }

        return $v;
    }

    $GLOBALS['records'] = [];
    ob_start(); // some .phpt files echo
    function record(array $r): void
    {
        if (isset($r['input']) && !mb_check_encoding($r['input'], 'UTF-8')) {
            $r['input_base64'] = base64_encode($r['input']);
            unset($r['input']);
        }
        $key = json_encode($r, JSON_INVALID_UTF8_SUBSTITUTE);
        $GLOBALS['records'][$key] = $r;
    }

    if (isset($argv[2])) {
        foreach (json_decode(file_get_contents($argv[2]), true) as $input) {
            try {
                Nette\Neon\Neon::decode($input);
            } catch (Throwable) {
            }
        }
    } else {
        foreach (glob($root.'/tests/Neon/*.phpt') as $file) {
            $code = file_get_contents($file);
            $code = str_replace("require __DIR__ . '/../bootstrap.php';", '', $code);
            $code = str_replace('__DIR__', var_export(dirname($file), true), $code);
            $tmp = tempnam(sys_get_temp_dir(), 'neon');
            file_put_contents($tmp, $code);
            try {
                (static function () use ($tmp): void {
                    require $tmp;
                })();
            } catch (Throwable $e) {
                fwrite(STDERR, basename($file).': '.$e->getMessage()."\n");
            }
            unlink($tmp);
        }
    }
    ob_end_clean();
    echo json_encode(array_values($GLOBALS['records']), JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_INVALID_UTF8_SUBSTITUTE), "\n";

    function test(Closure $function): void
    {
        $function();
    }
}

namespace Tester {
    final class Assert
    {
        public static function __callStatic(string $name, array $args): mixed
        {
            foreach ($args as $a) {
                if ($a instanceof \Closure) {
                    set_error_handler(static fn () => true);
                    try {
                        $a();
                    } catch (\Throwable) {
                    } finally {
                        restore_error_handler();
                    }
                }
            }

            return null;
        }
    }
}

namespace Nette\Neon {
    final class Neon
    {
        public const BLOCK = Encoder::BLOCK;
        public const Chain = '!!chain';
        public const CHAIN = self::Chain;

        public static function encode($value, bool $blockMode = false, string $indentation = "\t"): string
        {
            $encoder = new Encoder;
            $encoder->blockMode = $blockMode;
            $encoder->indentation = $indentation;
            $r = ['encode' => \tagged($value), 'block' => $blockMode, 'indentation' => $indentation];
            try {
                $out = $encoder->encode($value);
            } catch (\Throwable $e) {
                \record($r + ['error' => get_class($e)]);
                throw $e;
            }
            \record($r + ['output' => $out]);

            return $out;
        }

        public static function decode(string $input)
        {
            set_error_handler(static fn () => true, E_USER_DEPRECATED);
            try {
                $v = (new Decoder)->decode($input);
            } catch (Exception $e) {
                \record(['input' => $input, 'error' => $e->getMessage()]);
                throw $e;
            } finally {
                restore_error_handler();
            }
            \record(['input' => $input, 'value' => \tagged($v)]);

            return $v;
        }

        public static function decodeFile(string $file)
        {
            $input = @file_get_contents($file);
            if ($input === false) {
                throw new Exception("Unable to read file '$file'.");
            }
            if (substr($input, 0, 3) === "\u{FEFF}") {
                $input = substr($input, 3);
            }

            return self::decode($input);
        }
    }
}
