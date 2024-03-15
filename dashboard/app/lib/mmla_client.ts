import {auth} from "@/auth";

export class MMLAClient {

    private base_url: string;
    constructor(base_url: string) {
        this.base_url = base_url;
    }

    async protected_get<T>(url: string): Promise<T> {
        const session = await auth();
        const session_token = session?.apiToken;
        if (session_token) {
            const response = await fetch(this.base_url + url, {
                headers: {
                    'Authorization': 'Bearer ' + session_token
                }
            });
            if (response.ok) {
                return await response.json();
            } else {
                throw new Error('Failed to fetch');
            }
        }
        throw new Error('Failed to fetch');
    }
}