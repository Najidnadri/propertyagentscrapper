use std::{io::{Read, BufWriter}, time::Duration, thread::sleep};
use serde_json;
use tokio;
use thirtyfour::{self, DesiredCapabilities, WebDriver, By, Keys};
use rand::{self, prelude::SliceRandom};
use std::io::Write;

#[tokio::main]
async fn main() {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    ).unwrap();
    //let path = Path::new("./adblock.crx");
    //let _extension_result = caps.add_extension(path).unwrap();
    let driver = WebDriver::new("http://localhost:9515", &caps)
    .await
    .unwrap();

    driver.get("http://search.lppeh.gov.my").await.unwrap();

    let mut name_file = std::fs::OpenOptions::new().read(true).write(true).open("name.txt").expect("cannot open users file");
    let mut minpage_file = std::fs::OpenOptions::new().read(true).write(true).open("minpage.txt").expect("cannot open users file");
    let mut maxpage_file = std::fs::OpenOptions::new().read(true).write(true).open("maxpage.txt").expect("cannot open users file");
    let rawdata_file = std::fs::OpenOptions::new().read(true).write(true).append(true).open("rawdata.txt").expect("cannot open users file");

    let mut name_buffer = String::new();
    let _readed_name = name_file.read_to_string(&mut name_buffer).unwrap();

    let mut min_page = String::new();
    let _readed_name = minpage_file.read_to_string(&mut min_page).unwrap();

    let mut max_page = String::new();
    let _readed_name = maxpage_file.read_to_string(&mut max_page).unwrap();

    

    //setting
    let min_page = min_page.parse::<usize>().unwrap();
    let max_page = max_page.parse::<usize>().unwrap();
    
    driver.fullscreen_window().await.unwrap();
    
    for i in min_page ..= max_page {
        let collapse_header = driver.find_elements(By::ClassName("collapsible-header")).await.unwrap();
        let collapse_header = collapse_header[2].clone();
        collapse_header.click().await.unwrap();
        let input_id = driver.find_element(By::Id("ContentPlaceHolder1_txt_NegotiatorREN")).await.unwrap();
        let i_string = i.to_string();
        input_id.send_keys(Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace + Keys::Backspace).await.unwrap();
    
        input_id.send_keys(i_string).await.unwrap();
        
        let submit_button_element = driver.find_element(By::Id("ContentPlaceHolder1_btnSubmitNegotiator")).await.unwrap();
        submit_button_element.click().await.unwrap();

        //sleep
        let mut rng = rand::thread_rng();
        let mut nums: Vec<i32> = (3 ..= 4).collect();
        nums.shuffle(&mut rng);
        let n = nums[0] as u64;

        sleep(Duration::from_secs(n));

        //check if user exist
        match driver.find_elements(By::Tag("td")).await {
            Ok(elements) => {
                if elements.len() == 0 {
                    continue;
                }
                let ren_number = elements[1].clone().text().await.unwrap();
                let agent_name = elements[3].clone().text().await.unwrap();
                let company_name = elements[5].clone().text().await.unwrap();
                let phone_number = elements[7].clone().text().await.unwrap();
                let expiry_date = elements[9].clone().text().await.unwrap();

                let full_string = format!("{},{},{},{},{}", ren_number, agent_name, company_name, phone_number, expiry_date);
                println!("{}", full_string);
                let mut writer = BufWriter::new(&rawdata_file);
                writeln!(writer, "{}", full_string).expect("cannot write to rawdata.txt");
            },
            Err(_) => {
                continue;
            },
        }
    }

  
}


