<?php

require __DIR__.'/Capture.php';

return \Rector\Config\RectorConfig::configure()
    ->withRules([\RepeatedAssertionExpressions\Oracle\Capture::class])
    ->withBootstrapFiles([__DIR__.'/bootstrap.php'])
    ->withAutoloadPaths([dirname(__DIR__, 4).'/pun-lint/tests/phpunit-stubs'])
    ->withPhpVersion(80400)
    ->withCache(getenv('REPEATED_ASSERTION_CACHE'))
    ->withoutParallel();
