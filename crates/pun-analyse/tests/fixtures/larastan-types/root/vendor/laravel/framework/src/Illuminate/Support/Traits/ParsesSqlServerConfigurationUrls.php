<?php

namespace Illuminate\Support\Traits;

trait ParsesSqlServerConfigurationUrls
{
    /**
     * The SQL Server DSN option aliases.
     *
     * @var array<string, string>
     */
    protected static $sqlServerDsnOptionAliases = [
        'app' => 'appname',
        'authentication' => 'authentication',
        'columnencryption' => 'column_encryption',
        'connectionpooling' => 'pooling',
        'encrypt' => 'encrypt',
        'keystoreauthentication' => 'key_store_authentication',
        'keystoreprincipalid' => 'key_store_principal_id',
        'keystoresecret' => 'key_store_secret',
        'logintimeout' => 'login_timeout',
        'multipleactiveresultsets' => 'multiple_active_result_sets',
        'multisubnetfailover' => 'multi_subnet_failover',
        'transactionisolation' => 'transaction_isolation',
        'trustservercertificate' => 'trust_server_certificate',
    ];

    /**
     * Determine if the given value is a SQL Server DSN.
     *
     * @param  string  $url
     * @return bool
     */
    protected function isSqlServerDsn($url)
    {}

    /**
     * Parse a SQL Server DSN into a database configuration.
     *
     * @param  array<string, mixed>  $config
     * @param  string  $dsn
     * @return array<string, mixed>
     */
    protected function parseSqlServerDsnConfiguration($config, $dsn)
    {}

    /**
     * Get the database configuration options from a SQL Server DSN.
     *
     * @param  array<string, string>  $options
     * @return array<string, mixed>
     */
    protected function getSqlServerDsnConfigurationOptions($options)
    {}

    /**
     * Parse the options from a SQL Server DSN.
     *
     * @param  string  $dsn
     * @return array<string, string>
     */
    protected function parseSqlServerDsnOptions($dsn)
    {}

    /**
     * Parse the host and port from a SQL Server DSN Server option.
     *
     * @param  string|null  $server
     * @return array{0: string|null, 1: int|null}
     */
    protected function parseSqlServerDsnServer($server)
    {}
}
