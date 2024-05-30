use futures::StreamExt;
use ollama_rs::{generation::completion::{request::GenerationRequest, GenerationContext, GenerationFinalResponseData}, Ollama};

use tokio::io::AsyncWriteExt;
pub type Result<T> = core::result::Result<T, Error>;
const MODEL : &str = "llama3";
pub type Error = Box<dyn std::error::Error>;
#[tokio::main]
async fn main()->Result<()> {
    let ollama = Ollama::default();
    let mut context: Option<GenerationContext> = None;
    loop{
        let mut input = String::new();

        std::io::stdin().read_line(&mut input ).expect("Reading line failed");
        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit"){
            break
        }
        let mut gen_request = GenerationRequest::new(MODEL.to_string(), input.to_string());
        
        if let Some(context) = context.take(){
            gen_request = gen_request.context(context);
        }
        let mut final_data_list = gen_stream_print(&ollama, gen_request).await?;
        if let Some(final_data) = final_data_list.pop(){
            context = Some(final_data.context);
        }
        
        
        
        
    }
    Ok(())
    

}
pub async fn gen_stream_print(ollama:&Ollama,gen_request:GenerationRequest)->Result<Vec<GenerationFinalResponseData>>{
    let mut stream = ollama.generate_stream(gen_request).await.unwrap();
    let mut stdout = tokio::io::stdout();
    let mut final_data_response = Vec::new();
    while let Some(res)= stream.next().await {
        let responses = res.unwrap();
        for resp in responses{
            stdout.write(resp.response.as_bytes()).await.unwrap();
            stdout.flush().await.unwrap();
            if let Some(final_data) = resp.final_data{
                stdout.write_all(b"\n").await?;
				stdout.flush().await?;
                final_data_response.push(final_data);
                break;
            }
            
        }
    }
    stdout.write_all(b"\n").await?;
	stdout.flush().await?;
    Ok(final_data_response)
    


}
