// `pun migrate` (run with `php -r`): a .php-cs-fixer(.dist).php's rules as PHP-CS-Fixer's own ConfigurationResolver
// resolves them (rule validation, custom fixers), printed as a JSON object. On failure, its message and exit 1.
// argv: the project's vendor/autoload.php, the config path.
error_reporting(E_ALL & ~E_DEPRECATED & ~E_USER_DEPRECATED);
ob_start(static fn () => '', 4096); // the config's own output must not reach stdout

[, $autoload, $config] = $argv;
try {
    require $autoload;
    $resolver = new PhpCsFixer\Console\ConfigurationResolver(new PhpCsFixer\Config(), ['config' => $config, 'path' => [], 'path-mode' => 'override', 'dry-run' => true], getcwd(), new PhpCsFixer\ToolInfo());
    $resolver->getFixers();
    fwrite(STDOUT, json_encode((object) $resolver->getRules()));
} catch (Throwable $e) {
    fwrite(STDOUT, $e->getMessage());
    exit(1);
}
