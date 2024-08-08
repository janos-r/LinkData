type Joke = {
    date: string;
    amount: string;
    description: string;
};
type Data = {
    data: Joke[];
};

async function main() {
    const year = "2020";
    const resp = await fetch(
        `http://35.77.82.139:3000/transactions/?fromDate=${year}-01-01&toDate=${year}-01-05`,
    );
    const json: Data = await resp.json();
    const text = json.data.reduce((acu, curr) => {
        const split = curr.date.split("-");
        const day = split[0].padStart(2, "0");
        const monthN = +split[1] + 1; // zero indexed month
        const month = monthN.toString().padStart(2, "0");
        const amount = curr.amount.replace(",", "");
        return `${acu}\n${year}-${month}-${day},${amount},'${curr.description}'`;
    }, "date, amount, description");

    // @ts-ignore: Cannot find name 'Bun'
    Bun.write("output.csv", text);
}
main();
