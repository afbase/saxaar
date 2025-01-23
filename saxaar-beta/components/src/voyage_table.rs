use models::voyage::Voyage;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VoyageTableProps {
    pub voyages: Vec<Voyage>,
}

#[function_component(VoyageTable)]
pub fn voyage_table(props: &VoyageTableProps) -> Html {
    html! {
        <div class="mt-4 overflow-x-auto">
            <table class="min-w-full border-collapse border">
                <thead>
                    <tr class="bg-gray-100">
                        <th class="border p-2 text-left">{"Origin Port"}</th>
                        <th class="border p-2 text-left">{"Origin Region"}</th>
                        <th class="border p-2 text-left">{"Destination Port"}</th>
                        <th class="border p-2 text-left">{"Destination Region"}</th>
                        <th class="border p-2 text-left">{"Embarkation Date"}</th>
                        <th class="border p-2 text-left">{"Disembarkation Date"}</th>
                        <th class="border p-2 text-right">{"Slaves Embarked"}</th>
                        <th class="border p-2 text-right">{"Slaves Disembarked"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        props.voyages.iter().map(|voyage| {
                            html! {
                                <tr class="hover:bg-gray-50">
                                    <td class="border p-2">{format_option(voyage.origin_port)}</td>
                                    <td class="border p-2">{format_option(voyage.origin_region)}</td>
                                    <td class="border p-2">{format_option(voyage.destination_port)}</td>
                                    <td class="border p-2">{format_option(voyage.destination_region)}</td>
                                    <td class="border p-2">{voyage.embark_date.clone().unwrap_or_else(|| "Unknown".to_string())}</td>
                                    <td class="border p-2">{voyage.disembark_date.clone().unwrap_or_else(|| "Unknown".to_string())}</td>
                                    <td class="border p-2 text-right">{format_number(voyage.slaves_embarked)}</td>
                                    <td class="border p-2 text-right">{format_number(voyage.slaves_disembarked)}</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}

// Helper functions for formatting
fn format_option<T: std::fmt::Display>(value: Option<T>) -> String {
    match value {
        Some(v) => v.to_string(),
        None => "Unknown".to_string(),
    }
}

fn format_number(value: Option<i32>) -> String {
    match value {
        Some(v) => v.to_formatted_string(),
        None => "Unknown".to_string(),
    }
}

// Utility trait for number formatting
trait NumberFormat {
    fn to_formatted_string(&self) -> String;
}

impl NumberFormat for i32 {
    fn to_formatted_string(&self) -> String {
        let mut result = String::new();
        let num_str = self.to_string();
        let len = num_str.len();

        for (i, c) in num_str.chars().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                result.push(',');
            }
            result.push(c);
        }
        result
    }
}
