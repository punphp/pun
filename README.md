<p align="center"><img src="docs/public/og.png" width="600" alt="Pun: ultrafast unified PHP tooling"></p>

<p align="center">
<a href="https://packagist.org/packages/punphp/pun"><img src="https://img.shields.io/packagist/dt/punphp/pun" alt="Total Downloads"></a>
<a href="https://packagist.org/packages/punphp/pun"><img src="https://img.shields.io/packagist/v/punphp/pun" alt="Latest Stable Version"></a>
<a href="https://packagist.org/packages/punphp/pun"><img src="https://img.shields.io/packagist/l/punphp/pun" alt="License"></a>
</p>

## Introduction

Pun is an ultrafast, unified toolchain for PHP 8.4+ projects. One native binary formats, lints and analyses your code, configured by a single `pun.toml`. It formats code byte for byte like [Laravel Pint](https://github.com/laravel/pint), and `pun migrate` converts your existing Pint, PHP-CS-Fixer, Rector and PHPStan configs.

```sh
composer require --dev punphp/pun

vendor/bin/pun migrate   # or: vendor/bin/pun init
vendor/bin/pun check --fix
```

## Official Documentation

Documentation for Pun can be found on the [Pun website](https://pun.laravel.cloud/docs).

## Contributing

Thank you for considering contributing to Pun! Read [AGENTS.md](AGENTS.md) for how to build Pun (Rust 1.99 or later), the checks to run, and the upstream tools it is tested against. If Pun is missing a rule you rely on, please open an issue or send a pull request.

## Security Vulnerabilities

Please review [our security policy](https://github.com/punphp/pun/security/policy) on how to report security vulnerabilities.

## Credits

Pun would not be possible without [Mago](https://github.com/carthage-software/mago), [PHP-CS-Fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer), [Laravel Pint](https://github.com/laravel/pint), [PHPStan](https://github.com/phpstan/phpstan), [Larastan](https://github.com/larastan/larastan) and [Rector](https://github.com/rectorphp/rector). Pun uses Mago's parser and analysis libraries directly and ports behaviour, rules and test fixtures from these projects. Thank you to everyone behind them.

## License

Pun is open-sourced software licensed under the [MIT license](LICENSE). The licences of ported code are in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), and those of the bundled Rust crates in [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).
