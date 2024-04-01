const fs = require("fs");
const path = require("path");

const logFilePath = path.join(__dirname, "log.txt");

// Regular expression to match the command output block
const commandOutputRegex = /Command being timed: ".*?"[\s\S]*?Exit status: 0/g;

try {
  // Read the file synchronously
  const data = fs.readFileSync(logFilePath, "utf8");

  // Find all matches of the command output blocks
  const commandOutputs = data.match(commandOutputRegex);

  if (commandOutputs) {
    // Trim each command output block and store in an array
    const trimmedCommandOutputs = commandOutputs
      .map(mapShit)
      .filter((o) => o["User time (seconds)"] !== "0.00");

    const r = report(trimmedCommandOutputs);
    prettyPrintCombinedResultAsMarkdownTable(r);

    // Process the trimmed command outputs as needed
    prettyPrintCombinedResultAsTable(r);
  } else {
    console.log("No command outputs found in the log file.");
  }
} catch (err) {
  console.error("Error reading the log file:", err);
}

function mapShit(commandStr) {
  return commandStr
    .split("\n")
    .map((s) => s.trim())
    .filter((s) => !!s)
    .reduce((prev, cur) => {
      const split = cur.split(": ");
      if (split[1].startsWith('"')) {
        split[1] = split[1].slice(1, -1);
      }

      prev[split[0].trim()] = split[1].trim();

      return prev;
    }, {});
}

function report(arrOfCommandObjects) {
  const commandStats = {};

  // Helper function to convert elapsed time to seconds
  const elapsedToSeconds = (elapsed) => {
    const parts = elapsed.split(":").map((part) => {
      // Check if part is a number, if not, assume it's a single minute/second value and prepend a 0
      return isNaN(part) ? 0 : Number(part);
    });
    // Adjust calculation based on the number of parts (handles both 'm:ss' and 'h:mm:ss' formats)
    if (parts.length === 3) {
      return parts[0] * 3600 + parts[1] * 60 + parts[2];
    } else if (parts.length === 2) {
      return parts[0] * 60 + parts[1];
    } else {
      return parts[0];
    }
  };

  // Helper function to calculate statistics
  const calculateStats = (values) => {
    const min = Math.min(...values);
    const max = Math.max(...values);
    const mean = values.reduce((a, b) => a + b, 0) / values.length;
    const sortedValues = values.slice().sort((a, b) => a - b);
    const mid = Math.floor(sortedValues.length / 2);
    const median =
      sortedValues.length % 2 !== 0
        ? sortedValues[mid]
        : (sortedValues[mid - 1] + sortedValues[mid]) / 2;
    return { min, max, mean, median };
  };

  // Aggregate data by command
  arrOfCommandObjects.forEach((obj) => {
    const command = obj["Command being timed"];
    if (!commandStats[command]) {
      commandStats[command] = {
        userTime: [],
        systemTime: [],
        percentCPU: [],
        elapsedTime: [],
        maxResidentSetSize: [],
      };
    }
    commandStats[command].userTime.push(parseFloat(obj["User time (seconds)"]));
    commandStats[command].systemTime.push(
      parseFloat(obj["System time (seconds)"])
    );
    commandStats[command].percentCPU.push(
      parseFloat(obj["Percent of CPU this job got"].replace("%", ""))
    );
    commandStats[command].elapsedTime.push(
      elapsedToSeconds(obj["Elapsed (wall clock) time (h:mm:ss or m:ss)"])
    );
    commandStats[command].maxResidentSetSize.push(
      parseFloat(obj["Maximum resident set size (kbytes)"])
    );
  });

  // Calculate statistics for each command
  const statsReport = {};
  for (const command in commandStats) {
    statsReport[command.split("/").pop()] = {
      userTime: calculateStats(commandStats[command].userTime),
      systemTime: calculateStats(commandStats[command].systemTime),
      percentCPU: calculateStats(commandStats[command].percentCPU),
      elapsedTime: calculateStats(commandStats[command].elapsedTime),
      maxResidentSetSize: calculateStats(
        commandStats[command].maxResidentSetSize
      ),
    };
  }

  return statsReport;
}

function prettyPrintCombinedResultAsTable(result) {
  const headers = ["Command", "Metric", "Min", "Max", "Mean", "Median"];
  const columnWidths = [20, 20, 20, 20, 20, 20];

  // Create a header row
  const headerRow = headers
    .map((header, index) => header.padEnd(columnWidths[index]))
    .join("| ");
  console.log(`+${"-".repeat(headerRow.length + 1)}+`);
  console.log(`| ${headerRow}|`);
  console.log(`+${"-".repeat(headerRow.length + 1)}+`);

  // Collect all metrics for each command
  const metricsByType = {};

  // Iterate over each command to populate metricsByType
  for (const command in result) {
    for (const metric in result[command]) {
      if (!metricsByType[metric]) {
        metricsByType[metric] = [];
      }
      const stats = result[command][metric];
      metricsByType[metric].push({
        command,
        min: stats.min.toFixed(2),
        max: stats.max.toFixed(2),
        mean: stats.mean.toFixed(2),
        median: stats.median.toFixed(2),
      });
    }
  }

  // Iterate over each metric type and print the result rows for each command
  for (const metric in metricsByType) {
    metricsByType[metric].forEach((metricData) => {
      const row = [
        metricData.command.padEnd(columnWidths[0]),
        metric.padEnd(columnWidths[1]),
        metricData.min.padStart(columnWidths[2]),
        metricData.max.padStart(columnWidths[3]),
        metricData.mean.padStart(columnWidths[4]),
        metricData.median.padStart(columnWidths[5]),
      ].join("| ");

      console.log(`| ${row}|`);
    });
    console.log(`+${"-".repeat(headerRow.length + 1)}+`);
  }
}

function prettyPrintCombinedResultAsMarkdownTable(result) {
  const headers = ["Command", "Metric", "Min", "Max", "Mean", "Median"];
  let markdownTable = `| ${headers.join(" | ")} |\n`;
  markdownTable += `| ${headers.map(() => "---").join(" | ")} |\n`;

  // Collect all metrics for each command
  const metricsByType = {};

  // Iterate over each command to populate metricsByType
  for (const command in result) {
    for (const metric in result[command]) {
      if (!metricsByType[metric]) {
        metricsByType[metric] = [];
      }
      const stats = result[command][metric];
      metricsByType[metric].push({
        command,
        min: stats.min.toFixed(2),
        max: stats.max.toFixed(2),
        mean: stats.mean.toFixed(2),
        median: stats.median.toFixed(2),
      });
    }
  }

  // Iterate over each metric type and print the result rows for each command
  for (const metric in metricsByType) {
    metricsByType[metric].forEach((metricData) => {
      const row = [
        metricData.command,
        metric,
        metricData.min,
        metricData.max,
        metricData.mean,
        metricData.median,
      ].join(" | ");

      markdownTable += `| ${row} |\n`;
    });
  }

  // Replace the content under the heading "#### Metrics" in the README.md with this markdown table output
  let readmeContent = fs.readFileSync("../README.md", "utf8");
  readmeContent = readmeContent.replace(
    /(#### Metrics\n\n)[\s\S]*/,
    `$1${markdownTable}`
  );
  fs.writeFileSync("../README.md", readmeContent);
}
