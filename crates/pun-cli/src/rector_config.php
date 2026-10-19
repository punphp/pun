// `pun migrate` reads a rector.php through the project's own Rector and prints what Rector resolves as JSON: every rule
// it runs (sets included), the configured ones, withSkip() split into rules (class => null or paths) and paths, the
// paths, the set files and whether withPhpSets()/withPhpLevel() was used. Run as `php -r <this> -- <autoload> <config>`
// in the project root (withPhpSets() reads composer.json there). Errors print their message and exit 1.
error_reporting(E_ALL & ~E_DEPRECATED);
try {
    require $argv[1];
    $config = require $argv[2];
    if (!is_callable($config)) {
        throw new \RuntimeException('rector.php must return RectorConfig::configure() or a closure taking RectorConfig');
    }
    $rector = new \Rector\Config\RectorConfig();
    $config($rector);
    $parameter = fn (string $option): array => \Rector\Configuration\Parameter\SimpleParameterProvider::hasParameter($option)
        ? \Rector\Configuration\Parameter\SimpleParameterProvider::provideArrayParameter($option)
        : [];
    $builder = $config instanceof \Rector\Configuration\RectorConfigBuilder ? $config : null;
    $field = fn (string $name) => $builder === null ? null : (fn () => $this->$name)->call($builder);
    // Rector's SkippedClassResolver: a string key is a rule with its paths (null: everywhere); a list entry is a rule
    // when the class exists, else a path.
    $skipRules = [];
    $skipPaths = [];
    foreach ($parameter(\Rector\Configuration\Option::SKIP) as $key => $value) {
        if (is_string($key)) {
            $skipRules[$key] = $value === null ? null : array_values((array) $value);
        } elseif (is_string($value) && class_exists($value)) {
            $skipRules[$value] = null;
        } elseif (is_string($value)) {
            $skipPaths[] = $value;
        }
    }
    echo json_encode([
        'rules' => array_values($rector->getMainRectorClasses()),
        'configured' => array_keys($rector->getRuleConfigurations()),
        'skip_rules' => (object) $skipRules,
        'skip_paths' => $skipPaths,
        'paths' => array_values($parameter(\Rector\Configuration\Option::PATHS)),
        'sets' => array_values(array_unique($field('sets') ?? [])),
        'php_sets' => (bool) ($field('isWithPhpSetsUsed') || $field('isWithPhpLevelUsed')),
    ], JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR);
} catch (\Throwable $e) {
    echo $e->getMessage();
    exit(1);
}
