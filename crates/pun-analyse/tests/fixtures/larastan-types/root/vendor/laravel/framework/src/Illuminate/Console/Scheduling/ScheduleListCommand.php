<?php

namespace Illuminate\Console\Scheduling;

use Closure;
use Cron\CronExpression;
use DateTimeZone;
use Illuminate\Console\Command;
use Illuminate\Support\Arr;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use ReflectionClass;
use ReflectionFunction;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Terminal;

#[AsCommand(name: 'schedule:list')]
class ScheduleListCommand extends Command
{
    /**
     * The console command name.
     *
     * @var string
     */
    protected $signature = 'schedule:list
        {--timezone= : The timezone that times should be displayed in}
        {--environment=* : Display the tasks scheduled to run on this environment}
        {--next : Sort the listed tasks by their next due date}
        {--json : Output the scheduled tasks as JSON}
    ';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'List all scheduled tasks';

    /**
     * The terminal width resolver callback.
     *
     * @var \Closure|null
     */
    protected static $terminalWidthResolver;

    /**
     * Execute the console command.
     *
     * @param  \Illuminate\Console\Scheduling\Schedule  $schedule
     * @return void
     *
     * @throws \Exception
     */
    public function handle(Schedule $schedule)
    {}

    /**
     * Render the scheduled tasks information as JSON.
     *
     * @param  \Illuminate\Support\Collection  $events
     * @param  \DateTimeZone  $timezone
     * @return void
     */
    protected function displayJson(Collection $events, DateTimeZone $timezone)
    {}

    /**
     * Render the scheduled tasks information formatted for the CLI.
     *
     * @param  \Illuminate\Support\Collection  $events
     * @param  \DateTimeZone  $timezone
     * @return void
     */
    protected function displayForCli(Collection $events, DateTimeZone $timezone)
    {}

    /**
     * Get the spacing to be used on each event row.
     *
     * @param  \Illuminate\Support\Collection  $events
     * @return array<int, int>
     */
    private function getCronExpressionSpacing($events, DateTimeZone $timezone)
    {}

    /**
     * Get the spacing to be used on each event row.
     *
     * @param  \Illuminate\Support\Collection  $events
     * @return int
     */
    private function getRepeatExpressionSpacing($events)
    {}

    /**
     * List the given even in the console.
     *
     * @param  \Illuminate\Console\Scheduling\Event  $event
     * @param  int  $terminalWidth
     * @param  array  $expressionSpacing
     * @param  int  $repeatExpressionSpacing
     * @param  \DateTimeZone  $timezone
     * @param  string|null  $convertedExpression
     * @return array
     */
    private function listEvent($event, $terminalWidth, $expressionSpacing, $repeatExpressionSpacing, $timezone, $convertedExpression = null)
    {}

    /**
     * Get the repeat expression for an event.
     *
     * @param  \Illuminate\Console\Scheduling\Event  $event
     * @return string
     */
    private function getRepeatExpression($event)
    {}

    /**
     * Sort the events by due date if option set.
     *
     * @param  \Illuminate\Support\Collection  $events
     * @param  \DateTimeZone  $timezone
     * @return \Illuminate\Support\Collection
     */
    private function sortEvents(\Illuminate\Support\Collection $events, DateTimeZone $timezone)
    {}

    /**
     * Render the scheduled tasks information.
     *
     * @param  \Illuminate\Support\Collection  $events
     * @param  \DateTimeZone  $timezone
     * @return void
     */
    protected function display(Collection $events, DateTimeZone $timezone)
    {}

    /**
     * Get the next due date for an event.
     *
     * @param  \Illuminate\Console\Scheduling\Event  $event
     * @param  \DateTimeZone  $timezone
     * @return \Illuminate\Support\Carbon
     */
    private function getNextDueDateForEvent($event, DateTimeZone $timezone)
    {}

    /**
     * Format the cron expression based on the spacing provided.
     *
     * @param  string  $expression
     * @param  array<int, int>  $spacing
     * @return string
     */
    private function formatCronExpression($expression, $spacing)
    {}

    /**
     * Get the file and line number for the event closure.
     *
     * @param  \Illuminate\Console\Scheduling\CallbackEvent  $event
     * @return string
     */
    private function getClosureLocation(CallbackEvent $event)
    {}

    /**
     * Get the terminal width.
     *
     * @return int
     */
    public static function getTerminalWidth()
    {}

    /**
     * Set a callback that should be used when resolving the terminal width.
     *
     * @param  \Closure|null  $resolver
     * @return void
     */
    public static function resolveTerminalWidthUsing($resolver)
    {}
}
