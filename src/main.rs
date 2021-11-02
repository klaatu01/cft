use clap::{App, Arg};
use colored::*;
use colored_json::*;
use cw_parser::{parse_logs, Log, RawCloudWatchLog};
use minus::{page_all, Pager};
use std::convert::TryFrom;
use std::fmt::Write;
mod utils;

struct LogGroup {
    log_group_name: String,
    cloudwatch_logs: Vec<Log>,
}

async fn head_logs(stack: String, number: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = Pager::new().unwrap();
    if let Some(resources) = utils::describe_stack_resources(stack.clone()).await {
        let logs_groups: Vec<String> = resources
            .iter()
            .filter(|x| x.resource_type == "AWS::Logs::LogGroup")
            .map(|x| x.physical_resource_id.clone())
            .flatten()
            .collect();

        for logs_group in logs_groups.iter() {
            let l = utils::get_latest_logs(logs_group.to_string(), number).await;

            let cwl: Vec<RawCloudWatchLog> = l
                .as_ref()
                .unwrap()
                .iter()
                .map(|o| RawCloudWatchLog::try_from(o.clone()))
                .flatten()
                .collect();

            let parsed = parse_logs(cwl);

            writeln!(output, "{}", logs_group.as_str().red().bold().underline(),)?;
            for p in parsed.iter() {
                writeln!(
                    output,
                    "{}",
                    p.to_pretty_string().as_str().to_colored_json_auto()?,
                )?;
            }
        }
        minus::page_all(output)?;
        Ok(())
    } else {
        println!(
            "{}{}{}",
            "CloudFormation Stack: ".red(),
            stack.as_str().green().bold(),
            " could not be found".red()
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("cf-tools")
        .version("1.0")
        .author("Charles E. charlieede01@gmail.com")
        .about("Command line tools for CloudFormation")
        .subcommand(
            App::new("head-logs")
                .about(
                    "Quickly get the most recent logs for each function in a CloudFormation Stack",
                )
                .arg(Arg::new("number").short('n').default_value("20"))
                .arg("<INPUT>   'The arn or name of CloudFormation Stack'"),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("head-logs", sub_matches)) => {
            let number: usize = sub_matches.value_of("number").unwrap().parse().unwrap();
            let input = sub_matches.value_of("INPUT").unwrap();
            head_logs(input.into(), number).await
        }
        _ => Ok(()),
    }
}
