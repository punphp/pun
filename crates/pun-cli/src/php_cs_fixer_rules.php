// `pun migrate` (run with `php -r`): a .php-cs-fixer(.dist).php's rules as PHP-CS-Fixer's own ConfigurationResolver
// resolves them (rule validation, custom fixers), and its Finder, printed as one JSON object:
// {"rules": {...}, "finder": {"in": [...], "exclude": [...], "notName": [...], "notPath": [...], "unsupported": [...]}}.
// The Finder keys are pint.json's; `in` is relative to the project when inside it; `unsupported` names each Finder
// setting pun.toml has no key for. On failure, its message and exit 1.
// argv: the project's vendor/autoload.php, the config path.
error_reporting(E_ALL & ~E_DEPRECATED & ~E_USER_DEPRECATED);
ob_start(static fn () => '', 4096); // the config's own output must not reach stdout

[, $autoload, $config] = $argv;
try {
    require $autoload;
    $resolver = new PhpCsFixer\Console\ConfigurationResolver(new PhpCsFixer\Config(), ['config' => $config, 'path' => [], 'path-mode' => 'override', 'dry-run' => true], getcwd(), new PhpCsFixer\ToolInfo());
    $resolver->getFixers();
    $finder = $resolver->getConfig()->getFinder();
    $out = ['in' => [], 'exclude' => [], 'notName' => [], 'notPath' => [], 'unsupported' => []];
    if (!$finder instanceof Symfony\Component\Finder\Finder) {
        $out['unsupported'][] = 'a '.get_debug_type($finder).' as the Finder';
    } else {
        $get = static fn (string $p) => (new ReflectionProperty(Symfony\Component\Finder\Finder::class, $p))->getValue($finder);
        // Real paths on both sides: __DIR__ and getcwd() can differ by a symlink (macOS /var is /private/var).
        $root = rtrim(realpath(getcwd()), '/').'/';
        foreach ($get('dirs') as $dir) {
            $dir = rtrim(realpath($dir) ?: $dir, '/');
            if ($dir.'/' !== $root) {
                $out['in'][] = str_starts_with($dir, $root) ? substr($dir, strlen($root)) : $dir;
            }
        }
        // PhpCsFixer\Finder's own defaults: `*.php` files, `vendor` excluded (Pun's finder does both).
        $out['exclude'] = array_values(array_diff($get('exclude'), ['vendor']));
        $out['notName'] = array_values($get('notNames'));
        $out['notPath'] = array_values($get('notPaths'));
        foreach (array_diff($get('names'), ['/\.php$/']) as $n) {
            $out['unsupported'][] = 'name('.json_encode($n, JSON_UNESCAPED_SLASHES).')';
        }
        foreach (['paths' => 'path', 'depths' => 'depth', 'contains' => 'contains', 'notContains' => 'notContains', 'sizes' => 'size', 'dates' => 'date', 'filters' => 'filter', 'iterators' => 'append'] as $p => $call) {
            if ($get($p) !== []) {
                $out['unsupported'][] = $call.'()';
            }
        }
    }
    fwrite(STDOUT, json_encode(['rules' => (object) $resolver->getRules(), 'finder' => $out], JSON_UNESCAPED_SLASHES));
} catch (Throwable $e) {
    fwrite(STDOUT, $e->getMessage());
    exit(1);
}
