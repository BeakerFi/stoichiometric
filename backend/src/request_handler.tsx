import { exec } from 'child_process'

const DECODER = "src/apiDecoder/stoichiometric-decoder"

type DecodedLoan = {
    collateral_token: string,
    collateral_amount: number,
    amount_lent: number,
    loan_time: number,
    loan_to_value: number,
    interest_rate: number
}

async function decode_hex(mutable_data_hex:string, immutable_data_hex:string):Promise<{ stdout: string, stderr: string }> {
    return new Promise((resolve, reject) => {
        exec(`${DECODER} ${mutable_data_hex} ${immutable_data_hex}`, (error, stdout, stderr) => {
            if (error) {
                reject(error);
            } else {
                resolve({ stdout, stderr });
            }
        });
    });
}

export default async function decode_loan_nfr(mutable_data_hex:string,immutable_data_hex:string): Promise<DecodedLoan> {
    try {

        const decoded_data = await decode_hex(mutable_data_hex, immutable_data_hex)

        const [collateral_token,collateral_amount_str,amount_lent_str, loan_time_str,loan_to_value_str, interest_rate_str] = decoded_data.stdout.trim().split(" ")


        if (collateral_token == undefined){
            return Promise.reject("collateral_token Undefined")
        }

        if (collateral_amount_str == undefined){
            return Promise.reject("collateral_amount Undefined")
        }

        if (amount_lent_str == undefined){
            return Promise.reject("amount_lent Undefined")
        }

        if (loan_time_str == undefined){
            return Promise.reject("loan_time Undefined")
        }

        if (loan_to_value_str == undefined){
            return Promise.reject("loan_to_value Undefined")
        }

        if (interest_rate_str == undefined){
            return Promise.reject("interest_rate Undefined")
        }

        const collateral_amount = Number(collateral_amount_str)
        const amount_lent = Number(amount_lent_str)
        const loan_time = Number(loan_time_str)
        const loan_to_value = Number(loan_to_value_str)
        const interest_rate = Number(interest_rate_str)


        const data: DecodedLoan = {
            collateral_token,
            collateral_amount,
            amount_lent,
            loan_time,
            loan_to_value,
            interest_rate
        }

        return Promise.resolve(data)

    } catch (e) {
        console.log(e)
        return Promise.reject(e)
    }
}