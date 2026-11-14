<?php

// Runs a `Class::method` script for pun: argv is the vendor dir, the event name, 1 or 0 for dev mode, the callback,
// then the event's arguments. The callback gets the parts of Composer's script event that scripts use.

namespace Composer\IO {
    if (!interface_exists(IOInterface::class, false)) {
        interface IOInterface
        {
            public const QUIET = 1;
            public const NORMAL = 2;
            public const VERBOSE = 4;
            public const VERY_VERBOSE = 8;
            public const DEBUG = 16;
        }
    }

    final class PunIO implements IOInterface
    {
        public function isInteractive() { return false; }
        public function isVerbose() { return false; }
        public function isVeryVerbose() { return false; }
        public function isDebug() { return false; }
        public function isDecorated() { return false; }
        public function write($messages, $newline = true, $verbosity = self::NORMAL) { $this->out(STDOUT, $messages, $newline, $verbosity); }
        public function writeError($messages, $newline = true, $verbosity = self::NORMAL) { $this->out(STDERR, $messages, $newline, $verbosity); }
        public function ask($question, $default = null) { return $default; }
        public function askConfirmation($question, $default = true) { return $default; }
        public function askAndValidate($question, $validator, $attempts = null, $default = null) { return $default; }
        public function askAndHideAnswer($question) { return null; }
        public function select($question, $choices, $default, $attempts = false, $errorMessage = '', $multiselect = false) { return $default; }

        private function out($stream, $messages, $newline, $verbosity)
        {
            if ($verbosity > self::NORMAL) {
                return;
            }
            foreach ((array) $messages as $message) {
                fwrite($stream, strip_tags((string) $message).($newline ? PHP_EOL : ''));
            }
        }
    }
}

namespace Composer {
    final class PunConfig
    {
        private $values;
        public function __construct(array $values) { $this->values = $values; }
        public function get($key) { return $this->values[$key] ?? null; }
        public function has($key) { return array_key_exists($key, $this->values); }
        public function all() { return ['config' => $this->values]; }
    }

    final class PunPackage
    {
        private $json;
        public function __construct(array $json) { $this->json = $json; }
        public function getName() { return strtolower($this->json['name'] ?? '__root__'); }
        public function getPrettyName() { return $this->json['name'] ?? '__root__'; }
        public function getExtra() { return $this->json['extra'] ?? []; }
        public function getAutoload() { return $this->json['autoload'] ?? []; }
        public function getDevAutoload() { return $this->json['autoload-dev'] ?? []; }
        public function getScripts() { return $this->json['scripts'] ?? []; }
        public function getType() { return $this->json['type'] ?? 'library'; }
    }

    final class PunComposer
    {
        private $config;
        private $package;
        public function __construct(PunConfig $config, PunPackage $package) { $this->config = $config; $this->package = $package; }
        public function getConfig() { return $this->config; }
        public function getPackage() { return $this->package; }
    }
}

namespace Composer\Script {
    if (!class_exists(Event::class, false)) {
        class Event
        {
            private $name;
            private $composer;
            private $io;
            private $devMode;
            private $args;
            private $stopped = false;

            public function __construct($name, $composer, $io, $devMode, array $args)
            {
                $this->name = $name;
                $this->composer = $composer;
                $this->io = $io;
                $this->devMode = $devMode;
                $this->args = $args;
            }

            public function getName() { return $this->name; }
            public function getComposer() { return $this->composer; }
            public function getIO() { return $this->io; }
            public function isDevMode() { return $this->devMode; }
            public function getArguments() { return $this->args; }
            public function getFlags() { return []; }
            public function getOriginatingEvent() { return $this; }
            public function isPropagationStopped() { return $this->stopped; }
            public function stopPropagation() { $this->stopped = true; }
        }
    }
}

namespace {
    [, $vendorDir, $eventName, $devMode, $callback] = $argv;
    $args = array_slice($argv, 5);
    $root = json_decode((string) getenv('PUN_ROOT_PACKAGE'), true) ?: [];
    $config = ($root['config'] ?? []) + ['vendor-dir' => $vendorDir, 'bin-dir' => $vendorDir.'/bin'];
    $config['vendor-dir'] = $vendorDir;
    require_once $vendorDir.'/autoload.php';
    [$class, $method] = explode('::', $callback, 2);
    if (!class_exists($class)) {
        fwrite(STDERR, "Class $class is not autoloadable, can not call $eventName script".PHP_EOL);
        exit(0);
    }
    if (!is_callable($callback)) {
        fwrite(STDERR, "Method $callback is not callable, can not call $eventName script".PHP_EOL);
        exit(0);
    }
    $composer = new Composer\PunComposer(new Composer\PunConfig($config), new Composer\PunPackage($root));
    $event = new Composer\Script\Event($eventName, $composer, new Composer\IO\PunIO(), $devMode === '1', $args);
    exit($class::$method($event) === false ? 1 : 0);
}
