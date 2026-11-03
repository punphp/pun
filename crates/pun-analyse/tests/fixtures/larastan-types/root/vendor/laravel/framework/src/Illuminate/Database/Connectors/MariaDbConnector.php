<?php

namespace Illuminate\Database\Connectors;

use PDO;

class MariaDbConnector extends MySqlConnector
{
    /**
     * Get the sql_mode value.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return string|null
     */
    protected function getSqlMode(PDO $connection, array $config)
    {}
}
