//! Larastan's `SchemaAggregator` (src/Properties/SchemaAggregator.php, v3.14.0, itself from psalm/laravel-psalm-plugin):
//! the tables and columns the `up()` methods of the migrations build, function by function.

use std::collections::HashMap;

use mago_codex::metadata::CodebaseMetadata;
use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::strings::{class_basename, snake};

const SCHEMA_FACADE: &str = "Illuminate\\Support\\Facades\\Schema";
const BLUEPRINT: &str = "Illuminate\\Database\\Schema\\Blueprint";

/// `SchemaColumn`: a column's name, its readable type as a PHPDoc type string, and enum or set options.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Column {
    pub readable_type: String,
    pub nullable: bool,
    pub options: Option<Vec<String>>,
}

/// `SchemaTable`.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct Table {
    pub columns: HashMap<String, Column>,
}

impl Table {
    fn set_column(
        &mut self,
        name: &str,
        readable_type: &str,
        nullable: bool,
        options: Option<Vec<String>>,
    ) {
        let readable_type = readable_type.to_string();
        self.columns.insert(
            name.to_string(),
            Column {
                readable_type,
                nullable,
                options,
            },
        );
    }

    fn rename_column(&mut self, old: &str, new: &str) {
        if let Some(c) = self.columns.remove(old) {
            self.columns.insert(new.to_string(), c);
        }
    }

    fn drop_column(&mut self, name: &str) {
        self.columns.remove(name);
    }
}

/// `SchemaAggregator`. `model_key` answers `getModelReferenceType()`'s question about a model class: its table and
/// primary key name (Larastan instantiates the model for that).
pub(super) struct SchemaAggregator<'a> {
    pub tables: HashMap<String, Table>,
    codebase: &'a CodebaseMetadata,
    model_key: &'a dyn Fn(&str) -> Option<(String, String)>,
}

/// `$call->getArgs()[$i]->value`.
fn arg<'x, 'arena>(list: &'x ArgumentList<'arena>, i: usize) -> Option<&'x Expression<'arena>> {
    list.arguments.get(i).map(|a| match a {
        Argument::Positional(p) => p.value,
        Argument::Named(n) => n.value,
    })
}

/// The value of a `String_` node.
fn string<'arena>(e: Option<&Expression<'arena>>) -> Option<String> {
    match e? {
        Expression::Literal(Literal::String(s)) => {
            s.value.map(|v| String::from_utf8_lossy(v).into_owned())
        }
        _ => None,
    }
}

fn member_name<'x>(selector: &'x ClassLikeMemberSelector) -> Option<&'x str> {
    match selector {
        ClassLikeMemberSelector::Identifier(id) => std::str::from_utf8(id.value).ok(),
        _ => None,
    }
}

/// The string items of an `Array_` node.
fn string_items(e: &Expression) -> Option<Vec<String>> {
    let elements = match e {
        Expression::Array(a) => &a.elements,
        Expression::LegacyArray(a) => &a.elements,
        _ => return None,
    };
    Some(
        elements
            .iter()
            .filter_map(|el| string(el.get_value()))
            .collect(),
    )
}

/// `NodeFinder::findInstanceOf($stmts, Class_::class)`'s members, anonymous classes included.
#[derive(Default)]
struct Classes<'ast, 'arena>(Vec<&'ast Sequence<'arena, ClassLikeMember<'arena>>>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Classes<'ast, 'arena> {
    fn walk_in_class(&mut self, class: &'ast Class<'arena>, _: &mut ()) {
        self.0.push(&class.members);
    }
    fn walk_in_anonymous_class(&mut self, class: &'ast AnonymousClass<'arena>, _: &mut ()) {
        self.0.push(&class.members);
    }
}

/// `NodeFinder::findInstanceOf($node, Stmt\Expression::class)`.
#[derive(Default)]
struct ExpressionStatements<'ast, 'arena>(Vec<&'ast ExpressionStatement<'arena>>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for ExpressionStatements<'ast, 'arena> {
    fn walk_in_statement_expression(&mut self, s: &'ast ExpressionStatement<'arena>, _: &mut ()) {
        self.0.push(s);
    }
}

impl<'a> SchemaAggregator<'a> {
    pub(super) fn new(
        codebase: &'a CodebaseMetadata,
        model_key: &'a dyn Fn(&str) -> Option<(String, String)>,
    ) -> Self {
        SchemaAggregator {
            tables: HashMap::new(),
            codebase,
            model_key,
        }
    }

    fn is_a(&self, class: &str, parent: &str) -> bool {
        class.eq_ignore_ascii_case(parent)
            || self
                .codebase
                .is_instance_of(class.as_bytes(), parent.as_bytes())
    }

    /// `$class->toCodeString() === '\Schema'` or a subclass of the Schema facade.
    fn is_schema(&self, class: &Expression, names: &ResolvedNames) -> bool {
        let Expression::Identifier(id) = class else {
            return false;
        };
        let name = String::from_utf8_lossy(names.get(&id.span()));
        name == "Schema" || self.is_a(&name, SCHEMA_FACADE)
    }

    pub(super) fn add_statements(&mut self, program: &Program, names: &ResolvedNames) {
        let mut classes = Classes::default();
        classes.walk_program(program, &mut ());
        for members in classes.0 {
            self.add_class_statements(members, names);
        }
    }

    fn add_class_statements(&mut self, members: &Sequence<ClassLikeMember>, names: &ResolvedNames) {
        for member in members.iter() {
            let ClassLikeMember::Method(m) = member else {
                continue;
            };
            let MethodBody::Concrete(body) = &m.body else {
                continue;
            };
            if m.name.value == b"down" || body.statements.is_empty() {
                continue;
            }
            self.add_up_method_statements(body, names);
        }
    }

    fn add_up_method_statements(&mut self, body: &Block, names: &ResolvedNames) {
        let mut found = ExpressionStatements::default();
        found.walk_block(body, &mut ());
        for stmt in found.0 {
            let (name, args) = match stmt.expression {
                Expression::Call(Call::Method(call))
                    if matches!(call.object, Expression::Call(Call::StaticMethod(s))
                        if matches!(member_name(&s.method), Some("connection" | "setConnection"))
                            && self.is_schema(s.class, names)) =>
                {
                    (member_name(&call.method), &call.argument_list)
                }
                Expression::Call(Call::StaticMethod(call)) if self.is_schema(call.class, names) => {
                    (member_name(&call.method), &call.argument_list)
                }
                _ => continue,
            };
            match name {
                Some("create") => self.alter_table(args, true, names),
                Some("table") => self.alter_table(args, false, names),
                Some("drop" | "dropIfExists") => {
                    if let Some(table) = string(arg(args, 0)) {
                        self.tables.remove(&table);
                    }
                }
                Some("rename") => {
                    if let (Some(old), Some(new)) = (string(arg(args, 0)), string(arg(args, 1))) {
                        self.rename_table(&old, &new);
                    }
                }
                _ => {}
            }
        }
    }

    fn alter_table(&mut self, args: &ArgumentList, creating: bool, names: &ResolvedNames) {
        // ponytail: a `Foo::TABLE` class constant as the table name is not read
        let Some(table) = string(arg(args, 0)) else {
            return;
        };
        if creating {
            self.tables.insert(table.clone(), Table::default());
        }
        let Some(Expression::Closure(closure)) = arg(args, 1) else {
            return;
        };
        let Some(param) = closure.parameter_list.parameters.first() else {
            return;
        };
        if let Some(Hint::Identifier(id)) = &param.hint
            && !self.is_a(&String::from_utf8_lossy(names.get(&id.span())), BLUEPRINT)
        {
            return;
        }
        let statements = update_statements(&closure.body);
        self.process_column_updates(&table, param.variable.name, &statements, names);
    }

    fn process_column_updates(
        &mut self,
        table_name: &str,
        arg_name: &[u8],
        stmts: &[&ExpressionStatement],
        names: &ResolvedNames,
    ) {
        if !self.tables.contains_key(table_name) {
            return;
        }
        let mut table_name = table_name.to_string();
        for stmt in stmts {
            let Expression::Call(Call::Method(outer)) = stmt.expression else {
                continue;
            };
            if member_name(&outer.method).is_none() {
                continue;
            }
            let mut root = stmt.expression;
            let mut first = outer;
            let (mut nullable, mut unsigned) = (false, false);
            while let Expression::Call(Call::Method(call)) = root {
                match member_name(&call.method) {
                    Some("nullable") if nullable_argument_value(call) => nullable = true,
                    Some("unsigned") => unsigned = true,
                    _ => {}
                }
                first = call;
                root = call.object;
            }
            let Expression::Variable(Variable::Direct(var)) = root else {
                continue;
            };
            let Some(method) = member_name(&first.method) else {
                continue;
            };
            if var.name != arg_name {
                continue;
            }
            let first_arg = arg(&first.argument_list, 0);
            let second_arg = arg(&first.argument_list, 1);
            if method == "foreignIdFor" {
                let model = match first_arg {
                    Some(Expression::Access(Access::ClassConstant(c)))
                        if matches!(c.class, Expression::Identifier(_)) =>
                    {
                        format!("\\{}", String::from_utf8_lossy(names.get(&c.class.span())))
                    }
                    e => match string(e) {
                        Some(s) => s,
                        None => continue,
                    },
                };
                let column =
                    string(second_arg).unwrap_or_else(|| snake(class_basename(&model)) + "_id");
                let mut ty = self.model_reference_type(&model);
                if unsigned && ty.as_deref().is_none_or(|t| t == "int") {
                    ty = Some("non-negative-int".into());
                }
                let ty = ty.unwrap_or_else(|| "int".into());
                self.table(&table_name)
                    .set_column(&column, &ty, nullable, None);
                continue;
            }
            let column = match string(first_arg) {
                Some(column) => column,
                None => {
                    if method == "dropColumn"
                        && let Some(items) = first_arg.and_then(string_items)
                    {
                        let table = self.table(&table_name);
                        for c in items {
                            table.drop_column(&c);
                        }
                    }
                    if matches!(
                        method,
                        "timestamps"
                            | "timestampsTz"
                            | "nullableTimestamps"
                            | "nullableTimestampsTz"
                            | "rememberToken"
                    ) {
                        let table = self.table(&table_name);
                        match method.to_ascii_lowercase().as_str() {
                            "remembertoken" => {
                                table.set_column("remember_token", "string", nullable, None)
                            }
                            "timestamps" | "timestampstz" | "nullabletimestamps" => {
                                table.set_column("created_at", "string", true, None);
                                table.set_column("updated_at", "string", true, None);
                            }
                            _ => {}
                        }
                        continue;
                    }
                    let default = match method {
                        "softDeletes"
                        | "softDeletesTz"
                        | "softDeletesDatetime"
                        | "dropSoftDeletes"
                        | "dropSoftDeletesTz" => "deleted_at",
                        "uuid" => "uuid",
                        "id" => "id",
                        "ulid" => "ulid",
                        "ipAddress" => "ip_address",
                        "macAddress" => "mac_address",
                        _ => continue,
                    };
                    default.to_string()
                }
            };
            let options = second_arg.and_then(string_items);
            let alter = Alter {
                first: Some(first),
                column,
                nullable,
                unsigned,
                second_arg,
                options,
                stmt,
            };
            self.process_statement_alter_method(
                &method.to_ascii_lowercase(),
                alter,
                &mut table_name,
                names,
            );
        }
    }

    fn table(&mut self, name: &str) -> &mut Table {
        self.tables.entry(name.to_string()).or_default()
    }

    fn rename_table(&mut self, old: &str, new: &str) {
        if let Some(table) = self.tables.remove(old) {
            self.tables.insert(new.to_string(), table);
        }
    }

    fn model_reference_type(&self, model: &str) -> Option<String> {
        let (table, key) = (self.model_key)(model.trim_start_matches('\\'))?;
        Some(
            self.tables
                .get(&table)?
                .columns
                .get(&key)?
                .readable_type
                .clone(),
        )
    }

    fn process_statement_alter_method(
        &mut self,
        method: &str,
        a: Alter,
        table_name: &mut String,
        names: &ResolvedNames,
    ) {
        let table = self.table(table_name);
        let column = a.column.as_str();
        match method {
            "addcolumn" => {
                let first = a.first.map(|f| &f.argument_list);
                let ty = first.and_then(|l| string(arg(l, 0))).unwrap_or_default();
                let column = first.and_then(|l| string(arg(l, 1))).unwrap_or_default();
                let alter = Alter {
                    first: None,
                    column,
                    ..a
                };
                self.process_statement_alter_method(
                    &ty.to_ascii_lowercase(),
                    alter,
                    table_name,
                    names,
                );
            }
            "integer" | "tinyinteger" | "smallinteger" | "mediuminteger" | "biginteger" => {
                let ty = if a.unsigned {
                    "non-negative-int"
                } else {
                    "int"
                };
                table.set_column(column, ty, a.nullable, None);
            }
            "increments"
            | "id"
            | "unsignedbiginteger"
            | "unsignedinteger"
            | "unsignedmediuminteger"
            | "unsignedsmallinteger"
            | "unsignedtinyinteger"
            | "integerincrements"
            | "mediumincrements"
            | "smallincrements"
            | "tinyincrements"
            | "bigincrements"
            | "foreignid" => {
                table.set_column(column, "non-negative-int", a.nullable, None);
            }
            "char" | "datetimetz" | "date" | "datetime" | "foreignulid" | "foreignuuid"
            | "ipaddress" | "json" | "jsonb" | "linestring" | "longtext" | "macaddress"
            | "mediumtext" | "multilinestring" | "string" | "text" | "time" | "timestamp"
            | "ulid" | "uuid" | "binary" => {
                table.set_column(column, "string", a.nullable, None);
            }
            "boolean" => table.set_column(column, "bool", a.nullable, None),
            "geometry" | "geometrycollection" | "multipoint" | "multipolygon" | "multipolygonz"
            | "point" | "polygon" | "computed" => {
                table.set_column(column, "mixed", a.nullable, None)
            }
            "double" | "float" | "unsigneddecimal" | "decimal" => {
                table.set_column(column, "float", a.nullable, None)
            }
            "after" => {
                if let Some(Expression::Closure(closure)) = a.second_arg
                    && let Some(param) = closure.parameter_list.parameters.first()
                {
                    let stmts: Vec<_> = closure
                        .body
                        .statements
                        .iter()
                        .filter_map(|s| match s {
                            Statement::Expression(e) => Some(e),
                            _ => None,
                        })
                        .collect();
                    self.process_column_updates(table_name, param.variable.name, &stmts, names);
                }
            }
            "dropcolumn" | "dropifexists" | "dropsoftdeletes" | "dropsoftdeletestz"
            | "removecolumn" | "drop" => {
                table.drop_column(column);
            }
            "dropforeign" | "dropindex" | "dropprimary" | "dropunique" | "foreign" | "index"
            | "primary" | "renameindex" | "unique" | "dropspatialindex" => {}
            "dropmorphs" => {
                table.drop_column(&format!("{column}_type"));
                table.drop_column(&format!("{column}_id"));
            }
            "enum" => table.set_column(column, "enum", a.nullable, a.options),
            "morphs" => {
                table.set_column(&format!("{column}_type"), "string", a.nullable, None);
                table.set_column(&format!("{column}_id"), "int", a.nullable, None);
            }
            "nullablemorphs" => {
                table.set_column(&format!("{column}_type"), "string", true, None);
                table.set_column(&format!("{column}_id"), "int", true, None);
            }
            "nullableuuidmorphs" => {
                table.set_column(&format!("{column}_type"), "string", true, None);
                table.set_column(&format!("{column}_id"), "string", true, None);
            }
            "rename" => {
                if let Expression::Call(Call::Method(call)) = a.stmt.expression
                    && let Some(new) = string(arg(&call.argument_list, 0))
                {
                    self.rename_table(table_name, &new);
                    *table_name = new;
                }
            }
            "renamecolumn" => {
                if let Some(new) = string(a.second_arg) {
                    table.rename_column(column, &new);
                }
            }
            "set" => table.set_column(column, "set", a.nullable, a.options),
            "softdeletestz" | "timestamptz" | "timetz" | "year" | "softdeletes" => {
                table.set_column(column, "string", true, None);
            }
            "uuidmorphs" => {
                table.set_column(&format!("{column}_type"), "string", a.nullable, None);
                table.set_column(&format!("{column}_id"), "string", a.nullable, None);
            }
            // We know a property exists with a name, we just don't know its type.
            _ => table.set_column(column, "mixed", a.nullable, None),
        }
    }
}

/// `processStatementAlterMethod()`'s arguments.
struct Alter<'x, 'arena> {
    first: Option<&'x MethodCall<'arena>>,
    column: String,
    nullable: bool,
    unsigned: bool,
    second_arg: Option<&'x Expression<'arena>>,
    options: Option<Vec<String>>,
    stmt: &'x ExpressionStatement<'arena>,
}

/// `getNullableArgumentValue()`: `nullable()` and `nullable(<anything but a constant>)` are true, a constant only
/// when it is written `true`.
fn nullable_argument_value(call: &MethodCall) -> bool {
    match arg(&call.argument_list, 0) {
        None => true,
        Some(Expression::Literal(Literal::True(k))) => k.value == b"true",
        Some(
            Expression::Literal(Literal::False(_) | Literal::Null(_))
            | Expression::ConstantAccess(_),
        ) => false,
        Some(_) => true,
    }
}

/// `getUpdateStatements()`: the closure's statements, with the expression statements anywhere inside an `if` in
/// place of the `if`.
fn update_statements<'ast, 'arena>(
    body: &'ast Block<'arena>,
) -> Vec<&'ast ExpressionStatement<'arena>> {
    let mut out = Vec::new();
    for s in body.statements.iter() {
        match s {
            Statement::If(i) => {
                let mut found = ExpressionStatements::default();
                found.walk_if(i, &mut ());
                out.extend(found.0);
            }
            Statement::Expression(e) => out.push(e),
            _ => {}
        }
    }
    out
}
