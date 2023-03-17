import {
    RadixDappToolkit,
} from '@radixdlt/radix-dapp-toolkit'
import {dAppId} from "./constants";

function createPosition() {

}

function addToPosition() {

}

function claimFees() {

}

function removeLiquidity() {

}

function closePosition() {

}


const rdt = RadixDappToolkit(
    {dAppDefinitionAddress: dAppId, dAppName: 'stoichiometric.fi'},
    () => null,
    {networkId: 11}
);

function resetRdt() {
    localStorage.clear();
}

export {rdt, resetRdt, createPosition, addToPosition, claimFees, removeLiquidity, closePosition}

