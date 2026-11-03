<?php

namespace Illuminate\Database;

use Carbon\CarbonInterval;
use Closure;
use DateTimeInterface;
use Exception;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Database\Events\QueryExecuted;
use Illuminate\Database\Events\StatementPrepared;
use Illuminate\Database\Events\TransactionBeginning;
use Illuminate\Database\Events\TransactionCommitted;
use Illuminate\Database\Events\TransactionCommitting;
use Illuminate\Database\Events\TransactionRolledBack;
use Illuminate\Database\Query\Builder as QueryBuilder;
use Illuminate\Database\Query\Expression;
use Illuminate\Database\Query\Grammars\Grammar as QueryGrammar;
use Illuminate\Database\Query\Processors\Processor;
use Illuminate\Database\Schema\Builder as SchemaBuilder;
use Illuminate\Support\Arr;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Traits\Macroable;
use PDO;
use PDOStatement;
use RuntimeException;

use function Illuminate\Support\enum_value;

class Connection implements ConnectionInterface
{
    use DetectsConcurrencyErrors,
        DetectsLostConnections,
        Concerns\ManagesTransactions,
        InteractsWithTime,
        Macroable;

    /**
     * The active PDO connection.
     *
     * @var \PDO|(\Closure(): \PDO)|null
     */
    protected $pdo;

    /**
     * The active PDO connection used for reads.
     *
     * @var \PDO|(\Closure(): \PDO)|null
     */
    protected $readPdo;

    /**
     * The database connection configuration options for reading.
     *
     * @var array
     */
    protected $readPdoConfig = [];

    /**
     * The active PDO connection used for direct connections.
     *
     * @var \PDO|(\Closure(): \PDO)|null
     */
    protected $directPdo;

    /**
     * The database connection configuration options for direct connections.
     *
     * @var array
     */
    protected $directPdoConfig = [];

    /**
     * The name of the connected database.
     *
     * @var string
     */
    protected $database;

    /**
     * The type of the connection.
     *
     * @var string|null
     */
    protected $readWriteType;

    /**
     * The table prefix for the connection.
     *
     * @var string
     */
    protected $tablePrefix = '';

    /**
     * The database connection configuration options.
     *
     * @var array
     */
    protected $config = [];

    /**
     * The reconnector instance for the connection.
     *
     * @var (callable(\Illuminate\Database\Connection): mixed)
     */
    protected $reconnector;

    /**
     * Indicates if the connection is reconnecting after a lost connection.
     *
     * @var bool
     */
    protected $reconnecting = false;

    /**
     * The query grammar implementation.
     *
     * @var \Illuminate\Database\Query\Grammars\Grammar
     */
    protected $queryGrammar;

    /**
     * The schema grammar implementation.
     *
     * @var \Illuminate\Database\Schema\Grammars\Grammar|null
     */
    protected $schemaGrammar;

    /**
     * The query post processor implementation.
     *
     * @var \Illuminate\Database\Query\Processors\Processor
     */
    protected $postProcessor;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher|null
     */
    protected $events;

    /**
     * The default fetch mode of the connection.
     *
     * @var int
     */
    protected $fetchMode = PDO::FETCH_OBJ;

    /**
     * The number of active transactions.
     *
     * @var int
     */
    protected $transactions = 0;

    /**
     * The transaction manager instance.
     *
     * @var \Illuminate\Database\DatabaseTransactionsManager|null
     */
    protected $transactionsManager;

    /**
     * Indicates if changes have been made to the database.
     *
     * @var bool
     */
    protected $recordsModified = false;

    /**
     * Indicates if the connection should use the "write" PDO connection.
     *
     * @var bool
     */
    protected $readOnWriteConnection = false;

    /**
     * All of the queries run against the connection.
     *
     * @var array{query: string, bindings: array, time: float|null}[]
     */
    protected $queryLog = [];

    /**
     * Indicates whether queries are being logged.
     *
     * @var bool
     */
    protected $loggingQueries = false;

    /**
     * The duration of all executed queries in milliseconds.
     *
     * @var float
     */
    protected $totalQueryDuration = 0.0;

    /**
     * All of the registered query duration handlers.
     *
     * @var array{has_run: bool, handler: (callable(\Illuminate\Database\Connection, class-string<\Illuminate\Database\Events\QueryExecuted>): mixed)}[]
     */
    protected $queryDurationHandlers = [];

    /**
     * Indicates if the connection is in a "dry run".
     *
     * @var bool
     */
    protected $pretending = false;

    /**
     * All of the callbacks that should be invoked before a transaction is started.
     *
     * @var \Closure[]
     */
    protected $beforeStartingTransaction = [];

    /**
     * All of the callbacks that should be invoked before a query is executed.
     *
     * @var (\Closure(string, array, \Illuminate\Database\Connection): mixed)[]
     */
    protected $beforeExecutingCallbacks = [];

    /**
     * The connection resolvers.
     *
     * @var \Closure[]
     */
    protected static $resolvers = [];

    /**
     * The last retrieved PDO read / write type.
     *
     * @var null|'read'|'write'|'direct'
     */
    protected $latestPdoTypeRetrieved = null;

    /**
     * Create a new database connection instance.
     *
     * @param  \PDO|(\Closure(): \PDO)  $pdo
     * @param  string  $database
     * @param  string  $tablePrefix
     * @param  array  $config
     */
    public function __construct($pdo, $database = '', $tablePrefix = '', array $config = [])
    {}

    /**
     * Set the query grammar to the default implementation.
     *
     * @return void
     */
    public function useDefaultQueryGrammar()
    {}

    /**
     * Get the default query grammar instance.
     *
     * @return \Illuminate\Database\Query\Grammars\Grammar
     */
    protected function getDefaultQueryGrammar()
    {}

    /**
     * Set the schema grammar to the default implementation.
     *
     * @return void
     */
    public function useDefaultSchemaGrammar()
    {}

    /**
     * Get the default schema grammar instance.
     *
     * @return \Illuminate\Database\Schema\Grammars\Grammar|null
     */
    protected function getDefaultSchemaGrammar()
    {}

    /**
     * Set the query post processor to the default implementation.
     *
     * @return void
     */
    public function useDefaultPostProcessor()
    {}

    /**
     * Get the default post processor instance.
     *
     * @return \Illuminate\Database\Query\Processors\Processor
     */
    protected function getDefaultPostProcessor()
    {}

    /**
     * Get a schema builder instance for the connection.
     *
     * @return \Illuminate\Database\Schema\Builder
     */
    public function getSchemaBuilder()
    {}

    /**
     * Begin a fluent query against a database table.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Contracts\Database\Query\Expression|\UnitEnum|string  $table
     * @param  string|null  $as
     * @return \Illuminate\Database\Query\Builder
     */
    public function table($table, $as = null)
    {}

    /**
     * Get a new query builder instance.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    public function query()
    {}

    /**
     * Run a select statement and return a single result.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @param  bool  $useReadPdo
     * @return mixed
     */
    public function selectOne($query, $bindings = [], $useReadPdo = true)
    {}

    /**
     * Run a select statement and return the first column of the first row.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @param  bool  $useReadPdo
     * @return mixed
     *
     * @throws \Illuminate\Database\MultipleColumnsSelectedException
     */
    public function scalar($query, $bindings = [], $useReadPdo = true)
    {}

    /**
     * Run a select statement against the database.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @return array
     */
    public function selectFromWriteConnection($query, $bindings = [])
    {}

    /**
     * Run a select statement against the database.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @param  bool  $useReadPdo
     * @param  array  $fetchUsing
     * @return array
     */
    public function select($query, $bindings = [], $useReadPdo = true, array $fetchUsing = [])
    {}

    /**
     * Run a select statement against the database and returns all of the result sets.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @param  bool  $useReadPdo
     * @param  array  $fetchUsing
     * @return array
     */
    public function selectResultSets($query, $bindings = [], $useReadPdo = true, array $fetchUsing = [])
    {}

    /**
     * Run a select statement against the database and returns a generator.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @param  bool  $useReadPdo
     * @param  array  $fetchUsing
     * @return \Generator<int, \stdClass>
     */
    public function cursor($query, $bindings = [], $useReadPdo = true, array $fetchUsing = [])
    { yield; }

    /**
     * Configure the PDO prepared statement.
     *
     * @param  \PDOStatement  $statement
     * @return \PDOStatement
     */
    protected function prepared(PDOStatement $statement)
    {}

    /**
     * Get the PDO connection to use for a select query.
     *
     * @param  bool  $useReadPdo
     * @return \PDO
     */
    protected function getPdoForSelect($useReadPdo = true)
    {}

    /**
     * Run an insert statement against the database.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @return bool
     */
    public function insert($query, $bindings = [])
    {}

    /**
     * Run an update statement against the database.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @return int
     */
    public function update($query, $bindings = [])
    {}

    /**
     * Run a delete statement against the database.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @return int
     */
    public function delete($query, $bindings = [])
    {}

    /**
     * Execute an SQL statement and return the boolean result.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @return bool
     */
    public function statement($query, $bindings = [])
    {}

    /**
     * Run an SQL statement and get the number of rows affected.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @return int
     */
    public function affectingStatement($query, $bindings = [])
    {}

    /**
     * Run a raw, unprepared query against the PDO connection.
     *
     * @param  literal-string  $query
     * @return bool
     */
    public function unprepared($query)
    {}

    /**
     * Get the number of open connections for the database.
     *
     * @return int|null
     */
    public function threadCount()
    {}

    /**
     * Execute the given callback in "dry run" mode.
     *
     * @param  (\Closure(\Illuminate\Database\Connection): mixed)  $callback
     * @return array{query: string, bindings: array, time: float|null}[]
     */
    public function pretend(Closure $callback)
    {}

    /**
     * Execute the given callback without "pretending".
     *
     * @template TReturn
     *
     * @param  \Closure(): TReturn  $callback
     * @return TReturn
     */
    public function withoutPretending(Closure $callback)
    {}

    /**
     * Execute the given callback in "dry run" mode.
     *
     * @param  (\Closure(): (array{query: string, bindings: array, time: float|null}[]))  $callback
     * @return array{query: string, bindings: array, time: float|null}[]
     */
    protected function withFreshQueryLog($callback)
    {}

    /**
     * Bind values to their parameters in the given statement.
     *
     * @param  \PDOStatement  $statement
     * @param  array  $bindings
     * @return void
     */
    public function bindValues($statement, $bindings)
    {}

    /**
     * Prepare the query bindings for execution.
     *
     * @param  array  $bindings
     * @return array
     */
    public function prepareBindings(array $bindings)
    {}

    /**
     * Run a SQL statement and log its execution context.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @param  \Closure  $callback
     * @return mixed
     *
     * @throws \Illuminate\Database\QueryException
     */
    protected function run($query, $bindings, Closure $callback)
    {}

    /**
     * Run a SQL statement.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @param  \Closure  $callback
     * @return mixed
     *
     * @throws \Illuminate\Database\QueryException
     */
    protected function runQueryCallback($query, $bindings, Closure $callback)
    {}

    /**
     * Determine if the given database exception was caused by a unique constraint violation.
     *
     * @param  \Exception  $exception
     * @return bool
     */
    protected function isUniqueConstraintError(Exception $exception)
    {}

    /**
     * Extract the index and columns that caused a unique constraint violation.
     *
     * @param  Exception  $exception
     * @return array{index: string|null, columns: list<string>}
     */
    protected function parseUniqueConstraintViolation(Exception $exception): array
    {}

    /**
     * Log a query in the connection's query log.
     *
     * @param  string  $query
     * @param  array  $bindings
     * @param  float|null  $time
     * @return void
     */
    public function logQuery($query, $bindings, $time = null)
    {}

    /**
     * Get the elapsed time in milliseconds since a given starting point.
     *
     * @param  float  $start
     * @return float
     */
    protected function getElapsedTime($start)
    {}

    /**
     * Register a callback to be invoked when the connection queries for longer than a given amount of time.
     *
     * @param  \DateTimeInterface|\Carbon\CarbonInterval|float|int  $threshold
     * @param  (callable(\Illuminate\Database\Connection, \Illuminate\Database\Events\QueryExecuted): mixed)  $handler
     * @return void
     */
    public function whenQueryingForLongerThan($threshold, $handler)
    {}

    /**
     * Allow all the query duration handlers to run again, even if they have already run.
     *
     * @return void
     */
    public function allowQueryDurationHandlersToRunAgain()
    {}

    /**
     * Get the duration of all run queries in milliseconds.
     *
     * @return float
     */
    public function totalQueryDuration()
    {}

    /**
     * Reset the duration of all run queries.
     *
     * @return void
     */
    public function resetTotalQueryDuration()
    {}

    /**
     * Handle a query exception.
     *
     * @param  \Illuminate\Database\QueryException  $e
     * @param  string  $query
     * @param  array  $bindings
     * @param  \Closure  $callback
     * @return mixed
     *
     * @throws \Illuminate\Database\QueryException
     */
    protected function handleQueryException(QueryException $e, $query, $bindings, Closure $callback)
    {}

    /**
     * Handle a query exception that occurred during query execution.
     *
     * @param  \Illuminate\Database\QueryException  $e
     * @param  string  $query
     * @param  array  $bindings
     * @param  \Closure  $callback
     * @return mixed
     *
     * @throws \Illuminate\Database\QueryException
     */
    protected function tryAgainIfCausedByLostConnection(QueryException $e, $query, $bindings, Closure $callback)
    {}

    /**
     * Reconnect to the database.
     *
     * @return mixed|false
     *
     * @throws \Illuminate\Database\LostConnectionException
     */
    public function reconnect()
    {}

    /**
     * Reconnect to the database if a PDO connection is missing.
     *
     * @return void
     */
    public function reconnectIfMissingConnection()
    {}

    /**
     * Disconnect from the underlying PDO connection.
     *
     * @return void
     */
    public function disconnect()
    {}

    /**
     * Register a hook to be run just before a database transaction is started.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function beforeStartingTransaction(Closure $callback)
    {}

    /**
     * Register a hook to be run just before a database query is executed.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function beforeExecuting(Closure $callback)
    {}

    /**
     * Register a database query listener with the connection.
     *
     * @param  \Closure(\Illuminate\Database\Events\QueryExecuted): mixed  $callback
     * @return void
     */
    public function listen(Closure $callback)
    {}

    /**
     * Fire an event for this connection.
     *
     * @param  string  $event
     * @return array|null
     */
    protected function fireConnectionEvent($event)
    {}

    /**
     * Fire the given event if possible.
     *
     * @param  mixed  $event
     * @return void
     */
    protected function event($event)
    {}

    /**
     * Get a new raw query expression.
     *
     * @param  literal-string|int|float  $value
     * @return \Illuminate\Contracts\Database\Query\Expression
     */
    public function raw($value)
    {}

    /**
     * Escape a value for safe SQL embedding.
     *
     * @param  string|float|int|bool|null  $value
     * @param  bool  $binary
     * @return string
     *
     * @throws \RuntimeException
     */
    public function escape($value, $binary = false)
    {}

    /**
     * Escape a string value for safe SQL embedding.
     *
     * @param  string  $value
     * @return string
     */
    protected function escapeString($value)
    {}

    /**
     * Escape a boolean value for safe SQL embedding.
     *
     * @param  bool  $value
     * @return string
     */
    protected function escapeBool($value)
    {}

    /**
     * Escape a binary value for safe SQL embedding.
     *
     * @param  string  $value
     * @return string
     *
     * @throws \RuntimeException
     */
    protected function escapeBinary($value)
    {}

    /**
     * Determine if the database connection has modified any database records.
     *
     * @return bool
     */
    public function hasModifiedRecords()
    {}

    /**
     * Indicate if any records have been modified.
     *
     * @param  bool  $value
     * @return void
     */
    public function recordsHaveBeenModified($value = true)
    {}

    /**
     * Set the record modification state.
     *
     * @param  bool  $value
     * @return $this
     */
    public function setRecordModificationState(bool $value)
    {}

    /**
     * Reset the record modification state.
     *
     * @return void
     */
    public function forgetRecordModificationState()
    {}

    /**
     * Indicate that the connection should use the write PDO connection for reads.
     *
     * @param  bool  $value
     * @return $this
     */
    public function useWriteConnectionWhenReading($value = true)
    {}

    /**
     * Execute the given callback using the write PDO connection for reads.
     *
     * @template TReturn
     *
     * @param  (\Closure($this): TReturn)  $callback
     * @return TReturn
     */
    public function usingWritePdo(Closure $callback): mixed
    {}

    /**
     * Get the current PDO connection.
     *
     * @return \PDO
     */
    public function getPdo()
    {}

    /**
     * Get the current PDO connection parameter without executing any reconnect logic.
     *
     * @return \PDO|\Closure|null
     */
    public function getRawPdo()
    {}

    /**
     * Get the current PDO connection used for reading.
     *
     * @return \PDO
     */
    public function getReadPdo()
    {}

    /**
     * Get the current read PDO connection parameter without executing any reconnect logic.
     *
     * @return \PDO|\Closure|null
     */
    public function getRawReadPdo()
    {}

    /**
     * Get the current PDO connection used for direct connections.
     *
     * @return \PDO
     */
    public function getDirectPdo()
    {}

    /**
     * Get the current direct PDO connection parameter without executing any reconnect logic.
     *
     * @return \PDO|\Closure|null
     */
    public function getRawDirectPdo()
    {}

    /**
     * Set the PDO connection.
     *
     * @param  \PDO|\Closure|null  $pdo
     * @return $this
     */
    public function setPdo($pdo)
    {}

    /**
     * Set the PDO connection used for reading.
     *
     * @param  \PDO|\Closure|null  $pdo
     * @return $this
     */
    public function setReadPdo($pdo)
    {}

    /**
     * Set the read PDO connection configuration.
     *
     * @param  array  $config
     * @return $this
     */
    public function setReadPdoConfig(array $config)
    {}

    /**
     * Set the PDO connection used for direct connections.
     *
     * @param  \PDO|\Closure|null  $pdo
     * @return $this
     */
    public function setDirectPdo($pdo)
    {}

    /**
     * Set the direct PDO connection configuration.
     *
     * @param  array  $config
     * @return $this
     */
    public function setDirectPdoConfig(array $config)
    {}

    /**
     * Get the direct PDO connection configuration.
     *
     * @return array
     */
    public function getDirectPdoConfig()
    {}

    /**
     * Determine if this connection has a direct PDO connection configured.
     *
     * @return bool
     */
    public function hasDirectConnection()
    {}

    /**
     * Set the reconnect instance on the connection.
     *
     * @param  (callable(\Illuminate\Database\Connection): mixed)  $reconnector
     * @return $this
     */
    public function setReconnector(callable $reconnector)
    {}

    /**
     * Get the database connection name.
     *
     * @return string|null
     */
    public function getName()
    {}

    /**
     * Get the database connection with its read / write type.
     *
     * @return string|null
     */
    public function getNameWithReadWriteType()
    {}

    /**
     * Get an option from the configuration options.
     *
     * @param  string|null  $option
     * @return mixed
     */
    public function getConfig($option = null)
    {}

    /**
     * Get the basic connection information as an array for debugging.
     *
     * @return array
     */
    protected function getConnectionDetails()
    {}

    /**
     * Get the PDO driver name.
     *
     * @return string
     */
    public function getDriverName()
    {}

    /**
     * Get a human-readable name for the given connection driver.
     *
     * @return string
     */
    public function getDriverTitle()
    {}

    /**
     * Get the query grammar used by the connection.
     *
     * @return \Illuminate\Database\Query\Grammars\Grammar
     */
    public function getQueryGrammar()
    {}

    /**
     * Set the query grammar used by the connection.
     *
     * @param  \Illuminate\Database\Query\Grammars\Grammar  $grammar
     * @return $this
     */
    public function setQueryGrammar(Query\Grammars\Grammar $grammar)
    {}

    /**
     * Get the schema grammar used by the connection.
     *
     * @return \Illuminate\Database\Schema\Grammars\Grammar
     */
    public function getSchemaGrammar()
    {}

    /**
     * Set the schema grammar used by the connection.
     *
     * @param  \Illuminate\Database\Schema\Grammars\Grammar  $grammar
     * @return $this
     */
    public function setSchemaGrammar(Schema\Grammars\Grammar $grammar)
    {}

    /**
     * Get the query post processor used by the connection.
     *
     * @return \Illuminate\Database\Query\Processors\Processor
     */
    public function getPostProcessor()
    {}

    /**
     * Set the query post processor used by the connection.
     *
     * @param  \Illuminate\Database\Query\Processors\Processor  $processor
     * @return $this
     */
    public function setPostProcessor(Processor $processor)
    {}

    /**
     * Get the event dispatcher used by the connection.
     *
     * @return \Illuminate\Contracts\Events\Dispatcher|null
     */
    public function getEventDispatcher()
    {}

    /**
     * Set the event dispatcher instance on the connection.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     * @return $this
     */
    public function setEventDispatcher(Dispatcher $events)
    {}

    /**
     * Unset the event dispatcher for this connection.
     *
     * @return void
     */
    public function unsetEventDispatcher()
    {}

    /**
     * Run the statement to start a new transaction.
     *
     * @return void
     */
    protected function executeBeginTransactionStatement()
    {}

    /**
     * Set the transaction manager instance on the connection.
     *
     * @param  \Illuminate\Database\DatabaseTransactionsManager  $manager
     * @return $this
     */
    public function setTransactionManager($manager)
    {}

    /**
     * Unset the transaction manager for this connection.
     *
     * @return void
     */
    public function unsetTransactionManager()
    {}

    /**
     * Determine if the connection is in a "dry run".
     *
     * @return bool
     */
    public function pretending()
    {}

    /**
     * Get the connection query log.
     *
     * @return array{query: string, bindings: array, time: float|null}[]
     */
    public function getQueryLog()
    {}

    /**
     * Get the connection query log with embedded bindings.
     *
     * @return array
     */
    public function getRawQueryLog()
    {}

    /**
     * Clear the query log.
     *
     * @return void
     */
    public function flushQueryLog()
    {}

    /**
     * Enable the query log on the connection.
     *
     * @return void
     */
    public function enableQueryLog()
    {}

    /**
     * Disable the query log on the connection.
     *
     * @return void
     */
    public function disableQueryLog()
    {}

    /**
     * Determine whether we're logging queries.
     *
     * @return bool
     */
    public function logging()
    {}

    /**
     * Get the name of the connected database.
     *
     * @return string
     */
    public function getDatabaseName()
    {}

    /**
     * Set the name of the connected database.
     *
     * @param  string  $database
     * @return $this
     */
    public function setDatabaseName($database)
    {}

    /**
     * Set the read / write type of the connection.
     *
     * @param  string|null  $readWriteType
     * @return $this
     */
    public function setReadWriteType($readWriteType)
    {}

    /**
     * Retrieve the latest read / write type used.
     *
     * @return 'read'|'write'|'direct'|null
     */
    protected function latestReadWriteTypeUsed()
    {}

    /**
     * Get the table prefix for the connection.
     *
     * @return string
     */
    public function getTablePrefix()
    {}

    /**
     * Set the table prefix in use by the connection.
     *
     * @param  string  $prefix
     * @return $this
     */
    public function setTablePrefix($prefix)
    {}

    /**
     * Execute the given callback without table prefix.
     *
     * @template TReturn
     *
     * @param  (\Closure($this): TReturn)  $callback
     * @return TReturn
     */
    public function withoutTablePrefix(Closure $callback): mixed
    {}

    /**
     * Get the server version for the connection.
     *
     * @return string
     */
    public function getServerVersion(): string
    {}

    /**
     * Register a connection resolver.
     *
     * @param  string  $driver
     * @param  \Closure  $callback
     * @return void
     */
    public static function resolverFor($driver, Closure $callback)
    {}

    /**
     * Get the connection resolver for the given driver.
     *
     * @param  string  $driver
     * @return \Closure|null
     */
    public static function getResolver($driver)
    {}

    /**
     * Prepare the instance for cloning.
     *
     * @return void
     */
    public function __clone()
    {}
}
