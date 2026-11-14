<?php return array(
    'root' => array(
        'name' => 'acme/app',
        'pretty_version' => '1.0.0+no-version-set',
        'version' => '1.0.0.0',
        'reference' => null,
        'type' => 'project',
        'install_path' => __DIR__ . '/../../',
        'aliases' => array(),
        'dev' => true,
    ),
    'versions' => array(
        'acme/app' => array(
            'pretty_version' => '1.0.0+no-version-set',
            'version' => '1.0.0.0',
            'reference' => null,
            'type' => 'project',
            'install_path' => __DIR__ . '/../../',
            'aliases' => array(),
            'dev_requirement' => false,
        ),
        'acme/dev' => array(
            'pretty_version' => 'dev-main',
            'version' => 'dev-main',
            'reference' => 'def',
            'type' => 'library',
            'install_path' => __DIR__ . '/../acme/dev',
            'aliases' => array(
                0 => '2.x-dev',
            ),
            'dev_requirement' => true,
        ),
        'acme/lib' => array(
            'pretty_version' => 'v1.2',
            'version' => '1.2.0.0',
            'reference' => 'abc',
            'type' => 'library',
            'install_path' => __DIR__ . '/../acme/lib',
            'aliases' => array(),
            'dev_requirement' => false,
        ),
        'acme/meta' => array(
            'pretty_version' => '1.0.0',
            'version' => '1.0.0.0',
            'reference' => null,
            'type' => 'metapackage',
            'install_path' => null,
            'aliases' => array(),
            'dev_requirement' => false,
        ),
        'acme/old' => array(
            'dev_requirement' => false,
            'replaced' => array(
                0 => '^1',
                1 => 'v1.2',
            ),
        ),
        'psr/log-implementation' => array(
            'dev_requirement' => false,
            'provided' => array(
                0 => '1.0|2.0',
                1 => '3.0',
            ),
        ),
    ),
);
