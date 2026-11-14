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
        'acme/lib' => array(
            'pretty_version' => '1.2.0',
            'version' => '1.2.0.0',
            'reference' => '7aeb9537b4372d3b9b5a97c1da216175f3964bf7',
            'type' => 'library',
            'install_path' => __DIR__ . '/../acme/lib',
            'aliases' => array(),
            'dev_requirement' => false,
        ),
        'acme/old-tools' => array(
            'dev_requirement' => true,
            'replaced' => array(
                0 => '0.3.1',
            ),
        ),
        'acme/tools' => array(
            'pretty_version' => '0.3.1',
            'version' => '0.3.1.0',
            'reference' => 'a6d854dda27357c0e870f612995cbea12f20d200',
            'type' => 'library',
            'install_path' => __DIR__ . '/../acme/tools',
            'aliases' => array(),
            'dev_requirement' => true,
        ),
        'psr/log-implementation' => array(
            'dev_requirement' => false,
            'provided' => array(
                0 => '3.0',
            ),
        ),
    ),
);
