<?php

namespace Illuminate\Support;

use RuntimeException;

class Lottery
{
    /**
     * The number of expected wins.
     *
     * @var int|float
     */
    protected $chances;

    /**
     * The number of potential opportunities to win.
     *
     * @var int|null
     */
    protected $outOf;

    /**
     * The winning callback.
     *
     * @var null|callable
     */
    protected $winner;

    /**
     * The losing callback.
     *
     * @var null|callable
     */
    protected $loser;

    /**
     * The factory that should be used to generate results.
     *
     * @var callable|null
     */
    protected static $resultFactory;

    /**
     * Create a new Lottery instance.
     *
     * @param  int|float  $chances
     * @param  int<1, max>|null  $outOf
     *
     * @throws \RuntimeException
     */
    public function __construct($chances, $outOf = null)
    {}

    /**
     * Create a new Lottery instance.
     *
     * @param  int|float  $chances
     * @param  int|null  $outOf
     * @return static
     */
    public static function odds($chances, $outOf = null)
    {}

    /**
     * Set the winner callback.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function winner($callback)
    {}

    /**
     * Set the loser callback.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function loser($callback)
    {}

    /**
     * Run the lottery.
     *
     * @param  mixed  ...$args
     * @return mixed
     */
    public function __invoke(...$args)
    {}

    /**
     * Run the lottery.
     *
     * @param  null|int  $times
     * @return ($times is null ? mixed : list<mixed>)
     */
    public function choose($times = null)
    {}

    /**
     * Run the winner or loser callback, randomly.
     *
     * @param  mixed  ...$args
     * @return callable
     */
    protected function runCallback(...$args)
    {}

    /**
     * Determine if the lottery "wins" or "loses".
     *
     * @return bool
     */
    protected function wins()
    {}

    /**
     * The factory that determines the lottery result.
     *
     * @return callable
     */
    protected static function resultFactory()
    {}

    /**
     * Force the lottery to always result in a win.
     *
     * @param  callable|null  $callback
     * @return void
     */
    public static function alwaysWin($callback = null)
    {}

    /**
     * Force the lottery to always result in a loss.
     *
     * @param  callable|null  $callback
     * @return void
     */
    public static function alwaysLose($callback = null)
    {}

    /**
     * Set the sequence that will be used to determine lottery results.
     *
     * @param  array  $sequence
     * @param  callable|null  $whenMissing
     * @return void
     */
    public static function fix($sequence, $whenMissing = null)
    {}

    /**
     * Set the sequence that will be used to determine lottery results.
     *
     * @param  array  $sequence
     * @param  callable|null  $whenMissing
     * @return void
     */
    public static function forceResultWithSequence($sequence, $whenMissing = null)
    {}

    /**
     * Indicate that the lottery results should be determined normally.
     *
     * @return void
     */
    public static function determineResultsNormally()
    {}

    /**
     * Indicate that the lottery results should be determined normally.
     *
     * @return void
     */
    public static function determineResultNormally()
    {}

    /**
     * Set the factory that should be used to determine the lottery results.
     *
     * @param  callable  $factory
     * @return void
     */
    public static function setResultFactory($factory)
    {}
}
