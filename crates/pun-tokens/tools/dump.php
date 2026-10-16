<?php
// Token dump oracle for crates/pun-tokens (examples/parity.rs).
// Usage: php -d short_open_tag=0 dump.php raw|parse|fixer < list-of-files
//   raw   token_get_all($code)
//   parse token_get_all($code, TOKEN_PARSE)
//   fixer PhpCsFixer\Tokenizer\Tokens::fromCode($code) (needs PUN_PHP_CS_FIXER=<checkout>)
//   classy TokensAnalyzer::getClassyElements(): name=type, line=classIndex, text=index
// Output per file: "=== <path>\n", then per token "<name> <line> <len>\n<bytes>\n"
// (name "-" = plain-string token; line 0 when unknown), or "!!! <error>\n".
error_reporting(E_ALL & ~E_DEPRECATED);
$mode = $argv[1] ?? 'raw';
if ($mode === 'fixer' || $mode === 'classy') {
    require (getenv('PUN_PHP_CS_FIXER') ?: __DIR__.'/../../../.oracle/php-cs-fixer').'/vendor/autoload.php';
}
$out = fopen('php://stdout', 'wb');
while (($path = fgets(STDIN)) !== false) {
    $path = rtrim($path, "\n");
    if ($path === '') continue;
    $code = file_get_contents($path);
    fwrite($out, "=== $path\n");
    try {
        if ($mode === 'classy') {
            PhpCsFixer\Tokenizer\Tokens::clearCache();
            $a = new PhpCsFixer\Tokenizer\TokensAnalyzer(PhpCsFixer\Tokenizer\Tokens::fromCode($code));
            foreach ($a->getClassyElements() as $i => $e) {
                fwrite($out, $e['type'].' '.$e['classIndex'].' '.strlen((string) $i)."\n$i\n");
            }
        } elseif ($mode === 'fixer') {
            PhpCsFixer\Tokenizer\Tokens::clearCache();
            foreach (PhpCsFixer\Tokenizer\Tokens::fromCode($code) as $t) {
                $c = $t->getContent();
                fwrite($out, ($t->getName() ?? '-').' 0 '.strlen($c)."\n".$c."\n");
            }
        } else {
            foreach (token_get_all($code, $mode === 'parse' ? TOKEN_PARSE : 0) as $t) {
                [$name, $c, $line] = is_array($t) ? [token_name($t[0]), $t[1], $t[2]] : ['-', $t, 0];
                fwrite($out, "$name $line ".strlen($c)."\n".$c."\n");
            }
        }
    } catch (Throwable $e) {
        fwrite($out, '!!! '.str_replace("\n", ' ', get_class($e).': '.$e->getMessage())."\n");
    }
}
