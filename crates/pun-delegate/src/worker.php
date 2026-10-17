// Pun delegate worker (run with `php -r`): fixes files with the real PHP-CS-Fixer, one request at a time on stdin.
// argv: runtime (a Pint PHAR or a vendor/autoload.php), mode ("rules" | "config"), JSON rules or the config path.
// Replies are `<kind> <head> <len>\n<payload>`: H (handshake, JSON [name, priority] list), S (unchanged),
// C (changed; head = applied fixer indexes), R (the config's resolved rules), E (error: startup or this file), A (error: abort the run).
error_reporting(E_ALL & ~E_DEPRECATED & ~E_USER_DEPRECATED);
ini_set('display_errors', 'stderr');
ini_set('memory_limit', '-1');
ob_start(static fn () => '', 4096); // a fixer's echo must not reach the protocol (replies use fwrite(STDOUT))

use PhpCsFixer\Console\ConfigurationResolver;
use PhpCsFixer\Tokenizer\Tokens;

[, $runtime, $mode, $arg] = $argv;
$in = STDIN;
$send = static function (string $kind, string $head, string $body = ''): void {
    fwrite(STDOUT, $kind.' '.($head === '' ? '-' : $head).' '.strlen($body)."\n".$body);
    fflush(STDOUT);
};
$read = static function (int $n) use ($in): string {
    $s = '';
    while (strlen($s) < $n && !feof($in)) {
        $s .= fread($in, $n - strlen($s));
    }

    return $s;
};

try {
    if (str_ends_with($runtime, '.php')) {
        require $runtime;
    } else {
        Phar::loadPhar($runtime, 'pun-runtime.phar');
        require 'phar://pun-runtime.phar/vendor/autoload.php';
    }
    // PHP-CS-Fixer's own path (rule validation, custom fixers, whitespace, risky check, finder, rule customisers):
    // the .php-cs-fixer.php file, or the Config Pint's preset builds (rules, risky allowed, no config file).
    $resolver = $mode === 'config'
        ? new ConfigurationResolver(new PhpCsFixer\Config(), ['config' => $arg, 'path' => [], 'path-mode' => 'override', 'dry-run' => true], getcwd(), new PhpCsFixer\ToolInfo())
        : new ConfigurationResolver((new PhpCsFixer\Config())->setRules(json_decode($arg, true))->setRiskyAllowed(true), ['config' => ConfigurationResolver::IGNORE_CONFIG_FILE, 'allow-risky' => 'yes', 'path' => [], 'dry-run' => true], getcwd(), new PhpCsFixer\ToolInfo());
    $fixers = $resolver->getFixers();
    $customisers = $resolver->getRuleCustomisationPolicy()->getRuleCustomisers();
} catch (Throwable $e) {
    $send('E', '', $e->getMessage());
    exit(1);
}
$names = array_map(static fn ($f) => $f->getName(), $fixers);
$send('H', '', json_encode(array_map(static fn ($f) => [$f->getName(), $f->getPriority()], $fixers)));

while (($line = fgets($in)) !== false) {
    if ($line === "R\n") {
        $send('R', '', json_encode($resolver->getRules()));
        continue;
    }
    [, $plen, $clen] = explode(' ', rtrim($line));
    $path = $read((int) $plen);
    $code = $read((int) $clen);
    // Runner::fixFile, minus linting (Pun lints natively before and after) and event plumbing.
    try {
        $file = new SplFileInfo($path);
        $tokens = Tokens::fromCode($code);
        $ignored = class_exists(PhpCsFixer\Tokenizer\Analyzer\FixerAnnotationAnalyzer::class)
            ? ((new PhpCsFixer\Tokenizer\Analyzer\FixerAnnotationAnalyzer())->find($tokens)['php-cs-fixer-ignore'] ?? [])
            : [];
        $missing = array_diff($ignored, $names);
        if ($missing !== [] && !filter_var(getenv('PHP_CS_FIXER_IGNORE_MISMATCHED_RULES_EXCEPTIONS'), FILTER_VALIDATE_BOOLEAN)) {
            // Runner::validateRulesNamesForExceptions throws out of the whole run (A = abort), unlike fixer exceptions.
            $send('A', '', "@php-cs-fixer-ignore annotation(s) used for rules that are not in the current set of enabled rules:\n"
                .implode("\n", array_map(static fn ($n) => '- '.$n.($n === '' ? '(no name provided)' : ($n[0] === '@' ? ' (can exclude only rules, not sets)' : ' (unknown rule)')), $missing))
                ."\n\nPlease check your annotation(s) usage in {$path} to ensure that these rules are included, or update your annotation(s) usage if they have been replaced by other rules in the version of PHP CS Fixer you are using.");
            continue;
        }
        $old = $tokens->getCodeHash();
        $applied = [];
        foreach ($fixers as $i => $fixer) {
            if (in_array($fixer->getName(), $ignored, true)) {
                continue;
            }
            if (isset($customisers[$fixer->getName()])) {
                $actual = $customisers[$fixer->getName()]($file);
                if ($actual === false) {
                    continue;
                }
                $fixer = $actual === true ? $fixer : $actual;
            }
            if (!$fixer instanceof PhpCsFixer\AbstractFixer && (!$fixer->supports($file) || !$fixer->isCandidate($tokens))) {
                continue;
            }
            $fixer->fix($file, $tokens);
            if ($tokens->isChanged()) {
                $tokens->clearEmptyTokens();
                $tokens->clearChanged();
                $applied[] = $i;
            }
        }
        if ($applied !== [] && $tokens->getCodeHash() !== $old) {
            $send('C', implode(',', $applied), $tokens->generateCode());
        } else {
            $send('S', '');
        }
    } catch (Throwable $e) {
        $send('E', '', $e->getMessage());
    }
    Tokens::clearCache();
}
