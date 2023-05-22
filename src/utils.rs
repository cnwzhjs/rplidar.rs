use super::prelude::*;
use super::errors::*;
use std::f32::consts::PI;

const PI2:f32 = PI * 2f32;

fn find_first_valid_index(scan: &[ScanPoint]) -> Option<usize> {
    scan.iter().position(|p| p.is_valid())
}

fn find_last_valid_index(scan: &[ScanPoint]) -> Option<usize> {
    scan.iter().rev().position(|p| p.is_valid())

}

fn tune_head(scan: &mut [ScanPoint], inc_origin_angle: f32) -> Result<()> {
    if let Some(head_index) = find_first_valid_index(scan) {
        let mut i = head_index;

        while i != 0 {
            i -= 1;
            let mut expect_angle = scan[i + 1].angle() - inc_origin_angle;
            if expect_angle < 0f32 {
                expect_angle = 0f32;
            }
            scan[i].set_angle(expect_angle);
        }

        Ok(())
    } else {
        Err(RposError::OperationFail {
            description: "operation failed".to_owned()
        }.into())
    }
}

fn tune_tail(scan: &mut [ScanPoint], inc_origin_angle: f32) -> Result<()> {
    if let Some(tail_index) = find_last_valid_index(scan) {
        for i in tail_index+1..scan.len() {
            let mut expect_angle = scan[i - 1].angle() + inc_origin_angle;
            if expect_angle > PI2 {
                expect_angle -= PI2;
            }
            scan[i].set_angle(expect_angle);
        }

        Ok(())
    } else {
        Err(RposError::OperationFail {
            description: "operation failed".to_owned()
        }.into())
    }
}

/// sort scan points
pub fn sort_scan(scan: &mut Vec<ScanPoint>) -> Result<()> {
    if scan.is_empty() {
        return Ok(());
    }

    let inc_origin_angle = PI2 / (scan.len() as f32);
    
    tune_head(scan, inc_origin_angle)?;
    tune_tail(scan, inc_origin_angle)?;

    let front_angle = scan[0].angle();

    scan.iter_mut().enumerate().for_each(|(index, point)| {
        let mut expect_angle = front_angle + (index as f32) * inc_origin_angle;
            if expect_angle > PI2 {
                expect_angle -= PI2;
            }
            point.set_angle(expect_angle);
    });

    scan.sort();
    
    Ok(())
}
