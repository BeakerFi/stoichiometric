type token = {
    address: string,
    name: string, 
    symb: string, 
    icon_url: string
}

type pool = {
    token: token
}

export type {token, pool};